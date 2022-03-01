import React, { useEffect, useState } from "react";

import { useParams } from "react-router-dom";

import useDimensions from "react-cool-dimensions";

import Box from "@mui/material/Box";

import EditorMenu from "../components/editor-components/EditorMenu";
import EditorAppBar from "../components/editor-components/EditorAppBar";
import EditorModeButtons from "../components/editor-components/EditorModeButtons";
import TranslateModeButtons from "../components/editor-components/TranslateModeButtons";
import CameraSettingsButton from "../components/editor-components/CameraSettingsButton";
import GridSettingsButton from "../components/editor-components/GridSettingsButton";

import useNotification from "../services/hooks/useNotification";
import Texture from "../services/types/Texture";
import Prop from "../services/types/Prop";

import TextureImage from "../img/texture_thumbnails/red_brick_03.jpg";
import PropImage from "../img/prop_thumbnails/ottoman_01.png";

type EditorMode = "solid" | "face" | "vertex" | "prop";

export default function Editor() {
  // Get project ID
  const { projectId } = useParams<{ projectId: string }>();

  // Selected texture
  const [texture, setTexture] = useState<Texture>({
    id: 1,
    name: "Bricks",
    thumbnail: TextureImage,
    categories: ["Brick", "Dirty"],
  });
  const handleTextureChange = (texture: Texture) => {
    setTexture(texture);
  };

  // Selected prop
  const [prop, setProp] = useState<Prop>({
    id: 1,
    name: "Ottoman",
    thumbnail: PropImage,
    categories: ["Furniture", "Chair"],
  });
  const handlePropChange = (prop: Prop) => {
    setProp(prop);
  };

  // App bar button click
  const handleAppBarButtonClick = () => {};

  const [sender, setSender] = useState<any | null>(null);
  const [width, setWidth] = useState(1);
  const [height, setHeight] = useState(1);

  const { observe } = useDimensions({
    onResize: ({ width, height }) => {
      setWidth(Math.ceil(width));
      setHeight(Math.floor(height));
    },
  });

  useEffect(() => {
    if (sender !== null) {
      sender.setResolution(
        width * window.devicePixelRatio,
        height * window.devicePixelRatio
      );

      console.log("width: ", width, " height: ", height);
    }
  }, [width, height, sender]);

  useEffect(() => {
    import("viewport").then((viewport) => {
      const channel = new viewport.Channel();
      setSender(channel.sender());
      const callback = new viewport.Callback((_: any) => {});
      const resources = new viewport.Resources();
      viewport.run(channel, callback, resources);
    });
  }, []);

  //Editor mode
  const [editorMode, setEditorMode] = useState<EditorMode>("solid");
  const handleEditorModeChange = (e: any) => {
    if (e.target.value != null) {
      setEditorMode(e.target.value);
    }
  };

  //Camera speed
  const [cameraSpeed, setCameraSpeed] = useState(50);
  const handleCameraSpeedChange = (e: any) => {
    setCameraSpeed(e.target.value);
  };

  // Grid settings
  const [gridStep, setGridStep] = useState<number>(100);
  const handleGridStepChange = (e: any) => {
    setGridStep(e.target.value);
  };

  // Error display
  const { addNotification } = useNotification();

  return (
    <>
      <EditorAppBar onSave={handleAppBarButtonClick} />
      <Box width='100%' height='48px'></Box>

      <Box display='flex' height={`calc(100vh - 48px)`} overflow='hidden'>
        <Box width='100%' height='100%' ref={observe} bgcolor='#0c0c0c' />
        <EditorMenu
          texture={texture}
          handleTextureChange={handleTextureChange}
          prop={prop}
          handlePropChange={handlePropChange}
        />
      </Box>

      <canvas
        id='viewport-canvas'
        style={{
          position: "absolute",
          top: "48px",
        }}
        onContextMenu={(e) => {
          e.preventDefault();
        }}
      ></canvas>

      {/* viewport UI */}
      <EditorModeButtons
        editorMode={editorMode}
        handleEditorModeChange={handleEditorModeChange}
      />

      <TranslateModeButtons />

      <CameraSettingsButton
        cameraSpeed={cameraSpeed}
        handleCameraSpeedChange={handleCameraSpeedChange}
      />

      <GridSettingsButton
        gridStep={gridStep}
        handleGridStepChange={handleGridStepChange}
      />
    </>
  );
}
