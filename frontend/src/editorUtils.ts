import Environment from "./env";

interface Callbacks {
  editorModeChanged: (mode: number) => void;
  solidEditorModeChanged: (mode: number) => void;
  gizmoChanged: (gizmo: number) => void;
  cameraSpeedChanged: (speed: number) => void;
  gridSizeChanged: (size: number) => void;
  sceneSaved: (scene: Uint8Array, mode: string) => void;
}

interface AssetLoader {
  id: number;
  url: string;
}

export type { Callbacks };

export default class EditorHandle {
  private callbacks: Callbacks;
  private loopTimeout: NodeJS.Timeout | undefined;
  private actionQueue: any[];
  private lastExportMode: string;

  constructor(callbacks: Callbacks) {
    this.callbacks = callbacks;
    this.loopTimeout = undefined;
    this.actionQueue = [];
    this.lastExportMode = "";

    let initialized = false;

    import("viewport").then((module) => {
      this.loopTimeout = setInterval(() => {
        while (true) {
          const message = module.queryMessage();
          if (message !== undefined) {
            const json = JSON.parse(message);
            if (json !== undefined) {
              switch (json.message) {
                case "init": {
                  initialized = true;
                  break;
                }
                case "set-editor-mode": {
                  this.callbacks.editorModeChanged(json.mode);
                  break;
                }
                case "set-solid-editor-mode": {
                  this.callbacks.solidEditorModeChanged(json.mode);
                  break;
                }
                case "set-camera-speed": {
                  this.callbacks.cameraSpeedChanged(json.speed);
                  break;
                }
                case "set-grid-size": {
                  this.callbacks.gridSizeChanged(json.size);
                  break;
                }
              }
            }
          } else {
            break;
          }
        }

        if (!initialized) {
          return;
        }

        while (this.actionQueue.length > 0) {
          const action = this.actionQueue.pop();
          if (action !== undefined) {
            switch (action.type) {
              case "resolution": {
                module.setResolution(action.width, action.height);
                break;
              }
              case "texture-data": {
                module.textureData(action.id, action.data);
                console.log("texture-data");
                break;
              }
              case "load-textures": {
                module.loadTextures();
                console.log("load-textures");
                break;
              }
              case "prop-data": {
                module.propData(action.id, action.data);
                break;
              }
              case "load-props": {
                module.loadProps();
                break;
              }
              case "set-editor-mode": {
                module.setEditorMode(action.mode);
                break;
              }
              case "set-solid-editor-mode": {
                module.setSolidEditorMode(action.mode);
                break;
              }
              case "set-gizmo": {
                module.setGizmo(action.gizmo);
                break;
              }
              case "select-texture": {
                module.selectTexture(action.id);
                break;
              }
              case "select-prop": {
                module.selectProp(action.id);
                break;
              }
              case "set-camera-speed": {
                module.setCameraSpeed(action.speed);
                break;
              }
              case "save-scene": {
                module.saveScene();
                break;
              }
              case "set-grid-size": {
                module.setGridSize(action.size);
                break;
              }
              case "load-scene": {
                module.loadScene(action.scene);
                break;
              }
            }
          }
        }

        const savedScene = module.getSavedScene();
        if (savedScene !== undefined) {
          callbacks.sceneSaved(savedScene, this.lastExportMode);
        }
      }, 16);

      module.main();
    });
  }

  setResolution(width: number, height: number) {
    this.actionQueue.unshift({
      type: "resolution",
      width: width,
      height: height,
    });
  }

  loadTextures(loaders: AssetLoader[]) {
    Promise.all(
      loaders.map((loader) =>
        (async () => {
          const res = await fetch(`${Environment.asset_url}/${loader.url}`);
          const arrayBuffer = await res.arrayBuffer();
          this.actionQueue.unshift({
            type: "texture-data",
            id: loader.id,
            data: new Uint8Array(arrayBuffer),
          });
        })()
      )
    ).then(() => {
      this.actionQueue.unshift({
        type: "load-textures",
      });
    });
  }

  loadProps(loaders: AssetLoader[]) {
    Promise.all(
      loaders.map((loader) =>
        (async () => {
          const res = await fetch(`${Environment.asset_url}/${loader.url}`);
          const arrayBuffer = await res.arrayBuffer();
          this.actionQueue.unshift({
            type: "prop-data",
            id: loader.id,
            data: new Uint8Array(arrayBuffer),
          });
        })()
      )
    ).then(() => {
      this.actionQueue.unshift({
        type: "load-props",
      });
    });
  }

  setEditorMode(mode: number) {
    this.actionQueue.unshift({
      type: "set-editor-mode",
      mode: mode,
    });
  }

  setSolidEditorMode(mode: number) {
    this.actionQueue.unshift({
      type: "set-solid-editor-mode",
      mode: mode,
    });
  }

  setGizmo(gizmo: number) {
    this.actionQueue.unshift({
      type: "set-gizmo",
      gizmo: gizmo,
    });
  }

  saveScene(mode: string) {
    this.lastExportMode = mode;
    this.actionQueue.unshift({
      type: "save-scene",
    });
  }

  selectTexture(id: number) {
    this.actionQueue.unshift({
      type: "select-texture",
      id: id,
    });
  }

  selectProp(id: number) {
    this.actionQueue.unshift({
      type: "select-prop",
      id: id,
    });
  }

  setCameraSpeed(speed: number) {
    this.actionQueue.unshift({
      type: "set-camera-speed",
      speed: speed,
    });
  }

  setGridSize(size: number) {
    this.actionQueue.unshift({
      type: "set-grid-size",
      size: size,
    });
  }

  loadScene(scene: Uint8Array) {
    this.actionQueue.unshift({
      type: "load-scene",
      scene: scene,
    });
  }

  destroy() {
    if (this.loopTimeout !== undefined) {
      clearInterval(this.loopTimeout);
    }
  }
}
