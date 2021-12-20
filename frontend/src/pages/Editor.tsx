import { Box, Button } from "@mui/material";
import React, { useEffect } from "react";
import EditorMenu from "../components/editor-components/EditorMenu";
import EditorAppBar from "../components/editor-components/EditorAppBar";
import AppBarOffset from "../components/AppBarOffset";
import EditorHandle from "../EditorUtils";
import useDimensions from "react-cool-dimensions";
import Environment from "../env";

const appBarHeight = 48;
let editorHandle: EditorHandle;

export default function Editor() {
  const { observe } = useDimensions({
    onResize: ({ width, height }) => {
      editorHandle.setResolution(width, height);
    },
  });

  useEffect(() => {
    editorHandle = new EditorHandle();
  }, []);

  const handleClick = () => {
    editorHandle.loadTexture(0, `${Environment.asset_url}/vertex.png`);
    editorHandle.loadTexture(10, `${Environment.asset_url}/nodraw.png`);
  };

  return (
    <React.Fragment>
      <EditorAppBar />
      <AppBarOffset variant="dense" />
      <Box
        display="flex"
        height={`calc(100vh - ${appBarHeight}px)`}
        overflow="hidden"
      >
        <Box
          width="100%"
          height="100%"
          ref={observe}
          sx={{ backgroundColor: "#0c0c0c" }}
        ></Box>
        <EditorMenu />
      </Box>
      <canvas
        id="viewport-canvas"
        style={{ position: "absolute", top: `${appBarHeight}px` }}
        onContextMenu={(e) => {
          e.preventDefault();
        }}
      ></canvas>
      <Box position="absolute" top={`${appBarHeight}px`} left={10}>
        <Button onClick={handleClick}>CLICKME</Button>
      </Box>
    </React.Fragment>
  );
}
