import {
  Box,
  ButtonGroup,
  Button,
  Select,
  MenuItem,
  Tooltip,
  IconButton,
  Menu,
  Slider,
  List,
  ListItem,
  Divider,
  ToggleButtonGroup,
  ToggleButton,
} from "@mui/material";
import React, { useEffect, useState } from "react";
import EditorMenu from "../components/editor-components/EditorMenu";
import EditorAppBar from "../components/editor-components/EditorAppBar";
import AppBarOffset from "../components/AppBarOffset";
import EditorHandle from "../EditorUtils";
import useDimensions from "react-cool-dimensions";
import Environment from "../env";
import { Chair, VideoCameraBack, ViewCompact } from "@mui/icons-material";
import MeshSelectIcon from "../components/icons/MeshSelectIcon";
import FaceSelectIcon from "../components/icons/FaceSelectIcon";
import VertexSelectIcon from "../components/icons/VertexSelectIcon";

const appBarHeight = 48;
let editorHandle: EditorHandle;
type viewportMode = "solid" | "prop";
type selectionMode = "mesh" | "face" | "vertex";
type translateMode = "select" | "move" | "rotate" | "scale";

function editorModeChanged(mode: number) {
  console.log(`Editor mode was changed with hotkey (${mode})`);
}

function solidEditorModeChanged(mode: number) {
  console.log(`Solid editor mode was changed with hotkey (${mode})`);
}

export default function Editor() {
  const { observe } = useDimensions({
    onResize: ({ width, height }) => {
      editorHandle.setResolution(width, height);
    },
  });
  useEffect(() => {
    editorHandle = new EditorHandle({
      editorModeChanged: editorModeChanged,
      solidEditorModeChanged: solidEditorModeChanged,
    });

    editorHandle.textureData(0, `${Environment.asset_url}/vertex.png`);
    editorHandle.textureData(10, `${Environment.asset_url}/nodraw.png`);
    editorHandle.loadTextures();
    return editorHandle.destroy;
  }, []);

  // Viewport mode change
  const [viewportMode, setViewportMode] = useState<viewportMode>("solid");
  const handleViewportModeChange = (e: any) => {
    setViewportMode(e.target.value);
  };

  // Selection mode change
  const [selectionMode, setSelectionMode] =
    React.useState<selectionMode>("mesh");

  const handleSelectionModeChange = (
    event: React.MouseEvent<HTMLElement>,
    newSelectionMode: selectionMode
  ) => {
    setSelectionMode(newSelectionMode);
  };
  // Translate mode change
  const [translateMode, setTranslateMode] =
    React.useState<translateMode>("select");

  const handleTranslateModeChange = (
    event: React.MouseEvent<HTMLElement>,
    newTranslateMode: translateMode
  ) => {
    setTranslateMode(newTranslateMode);
  };

  // Camera settings dropdown
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const cameraMenuOpen = Boolean(anchorEl);
  const handleCameraMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleCameraMenuClose = () => {
    setAnchorEl(null);
  };

  return (
    <React.Fragment>
      <EditorAppBar />
      <AppBarOffset variant='dense' />
      <Box
        display='flex'
        height={`calc(100vh - ${appBarHeight}px)`}
        overflow='hidden'
      >
        <Box
          width='100%'
          height='100%'
          ref={observe}
          sx={{ backgroundColor: "#0c0c0c" }}
        ></Box>
        <EditorMenu />
      </Box>
      <canvas
        id='viewport-canvas'
        style={{ position: "absolute", top: `${appBarHeight}px` }}
        onContextMenu={(e) => {
          e.preventDefault();
        }}
      ></canvas>

      {/* viewport UI */}
      <>
        {/* Viewport mode */}
        <Box position='absolute' top={appBarHeight + 10} left={10} width={180}>
          <Select
            id='mode-select'
            value={viewportMode}
            onChange={handleViewportModeChange}
            size='small'
            fullWidth
            sx={{
              color: "white",
              height: 30.75,
              underline: {
                "&:after": {
                  borderBottom: "1px solid pink",
                  borderTop: "1px solid pink",
                },
              },
            }}
          >
            <MenuItem value='prop'>
              <Box display='flex' alignItems='center' gap={2}>
                <Chair fontSize='small' /> Prop mode
              </Box>
            </MenuItem>
            <MenuItem value='solid'>
              <Box display='flex' alignItems='center' gap={2}>
                <ViewCompact fontSize='small' /> Solid mode
              </Box>
            </MenuItem>
          </Select>
        </Box>

        {/* Selection mode */}
        <Box
          position='absolute'
          top={appBarHeight + 10}
          left={220}
          display={viewportMode === "solid" ? "initial" : "none"}
        >
          <ToggleButtonGroup
            value={selectionMode}
            exclusive
            onChange={handleSelectionModeChange}
            color='primary'
            size='small'
            sx={{ height: 30.75 }}
          >
            <ToggleButton value='mesh'>
              <Tooltip title='Mesh select mode'>
                <Box marginTop={0.8}>
                  <MeshSelectIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='face'>
              <Tooltip title='Face select mode'>
                <Box marginTop={0.8}>
                  <FaceSelectIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='vertex'>
              <Tooltip title='Vertex select mode'>
                <Box marginTop={0.8}>
                  <VertexSelectIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
          </ToggleButtonGroup>
        </Box>

        {/* Translate mode */}
        <Box position='absolute' top='40vh' left={10}>
          <ToggleButtonGroup
            value={translateMode}
            exclusive
            onChange={handleTranslateModeChange}
            color='primary'
            size='small'
            orientation='vertical'
          >
            <ToggleButton value='select'>
              <Tooltip title='Select' placement='right'>
                <Box marginTop={0.8} width={36} height={30}>
                  <VertexSelectIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='move'>
              <Tooltip title='Move' placement='right'>
                <Box marginTop={0.8} width={36} height={30}>
                  <VertexSelectIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='rotate'>
              <Tooltip title='Rotate' placement='right'>
                <Box marginTop={0.8} width={36} height={30}>
                  <VertexSelectIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='scale'>
              <Tooltip title='Scale' placement='right'>
                <Box marginTop={0.8} width={36} height={30}>
                  <VertexSelectIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
          </ToggleButtonGroup>
        </Box>

        {/* Camera settings */}
        <Box
          position='absolute'
          top={appBarHeight + 10}
          left='calc(100% - 400px)'
        >
          <Tooltip title='Camera settings'>
            <IconButton onClick={handleCameraMenuClick}>
              <VideoCameraBack />
            </IconButton>
          </Tooltip>
        </Box>
        <Menu
          anchorEl={anchorEl}
          id='account-menu'
          open={cameraMenuOpen}
          onClose={handleCameraMenuClose}
          PaperProps={{
            elevation: 0,
            sx: {
              overflow: "visible",
              filter: "drop-shadow(0px 2px 8px rgba(0,0,0,0.5))",
              mt: 1.5,
              borderRadius: 2,
              "&:before": {
                content: '""',
                display: "block",
                position: "absolute",
                top: 0,
                right: 25,
                width: 10,
                height: 10,
                bgcolor: "paper.background",
                transform: "translateY(-50%) rotate(45deg)",
                zIndex: 0,
              },
            },
          }}
          transformOrigin={{ horizontal: "right", vertical: "top" }}
          anchorOrigin={{ horizontal: "right", vertical: "bottom" }}
        >
          <List dense>
            <ListItem>
              <Box>
                Camera speed
                <Box width={150}>
                  <Slider
                    size='small'
                    defaultValue={50}
                    valueLabelDisplay='auto'
                  />
                </Box>
              </Box>
            </ListItem>
          </List>
        </Menu>
      </>
    </React.Fragment>
  );
}
