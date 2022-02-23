import React from "react";

import { useTranslation } from "react-i18next";

import { styled } from "@mui/material/styles";

import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import Tooltip from "@mui/material/Tooltip";
import Button from "@mui/material/Button";

import Logo from "../general-components/Logo";

import { ColorMode, useColorMode } from "../../services/colorMode";
import LanguageSelectDropdown from "../general-components/LanguageSelectDropdown";
import DarkModeSwitch from "../general-components/DarkModeSwitch";

const CustomEditorAppBar = styled(AppBar)(({ theme }) => ({
  zIndex: theme.zIndex.drawer + 1,
  backgroundColor: theme.palette.background.paper,
}));

interface EditorAppBarProps {
  onSave: (type: "export" | "save" | "render") => void;
}

export default function EditorAppBar({ onSave }: EditorAppBarProps) {
  const { t } = useTranslation();
  const tooltipText: string =
    t("archytex") + " " + t("version") + " " + t("version_number");

  const [colorMode, _] = useColorMode();

  return (
    <CustomEditorAppBar elevation={0}>
      <Toolbar
        variant='dense'
        sx={{
          borderBottom:
            colorMode === ColorMode.Dark
              ? "1px solid #2E2E2E"
              : "1px solid #BABABA",
        }}
      >
        <Box
          display='flex'
          justifyContent='space-between'
          width='100%'
          alignItems='center'
        >
          <Box width='100%' height='100%' display='flex' alignItems='center'>
            <Box height='100%' display='flex' alignItems='center'>
              <Tooltip title={tooltipText} placement='bottom-start'>
                <Box>
                  <Logo />
                </Box>
              </Tooltip>
            </Box>

            {/* Appbar menu */}
            <Box display='flex'>
              <Button
                variant='text'
                color='inherit'
                sx={{ textTransform: "none" }}
                onClick={() => onSave("save")}
              >
                {t("save")}
              </Button>
              <Button
                variant='text'
                color='inherit'
                sx={{ textTransform: "none" }}
                onClick={() => onSave("export")}
              >
                {t("export")}
              </Button>
              <Button
                variant='text'
                color='inherit'
                sx={{ textTransform: "none" }}
                onClick={() => onSave("render")}
              >
                {t("render")}
              </Button>
            </Box>
          </Box>
          <Box alignSelf='right' display='flex' flexWrap='nowrap'>
            <LanguageSelectDropdown />
            <DarkModeSwitch />
          </Box>
        </Box>
      </Toolbar>
    </CustomEditorAppBar>
  );
}
