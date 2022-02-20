import React, { useState } from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import Fade from "@mui/material/Fade";
import Modal from "@mui/material/Modal";
import Backdrop from "@mui/material/Backdrop";

import { Close } from "@mui/icons-material";

import { Project, useProjects } from "../../../../services/projects";

import FormInput from "../../../form-components/FormInput";

interface Props {
  project: Project;
  open: boolean;
  handleClose: () => void;
}

export default function EditProjectModal({
  project,
  open,
  handleClose,
}: Props) {
  const { t } = useTranslation();

  const [title, setTitle] = useState(project.title);

  const [error, setError] = useState("");

  const { dispatch: projectsDispatch } = useProjects();

  const handleSaveEdit = () => {
    if (title.trim() === "") {
      setError(t("no_empty_project_name"));
      return;
    }

    projectsDispatch({
      id: project.id,
      type: "rename",
      name: title,
    }).catch((error) => {
      setError(error.message);
      return;
    });

    handleClose();
  };

  return (
    <Modal
      open={open}
      onClose={handleClose}
      closeAfterTransition
      BackdropComponent={Backdrop}
      BackdropProps={{
        timeout: 500,
      }}
    >
      <Fade in={open}>
        <Box
          sx={{
            position: "absolute" as "absolute",
            top: "50%",
            left: "50%",
            transform: "translate(-50%, -50%)",
            width: { xs: 400, sm: 500, md: 600, lg: 600 },
            bgcolor: "background.paper",
            filter: "drop-shadow(0px 0px 4px rgba(0,0,0,0.5))",
            boxShadow: 24,
            p: 4,
            borderRadius: 2,
          }}
          borderRadius={4}
          display='flex'
          flexDirection='column'
          justifyContent='space-between'
        >
          <Box display='flex' justifyContent='space-between'>
            <Typography id='transition-modal-title' variant='h6' component='h2'>
              {t("edit_project_name")}
            </Typography>
            <IconButton onClick={handleClose}>
              <Close />
            </IconButton>
          </Box>
          <Box display='flex' flexDirection='column' marginBottom={3}>
            <FormInput
              variant={"regular"}
              label={t("project_name")}
              input={title}
              inputChange={(e) => setTitle(e.target.value)}
              error={error}
            />
          </Box>
          <Box>
            <Button
              type='submit'
              size='large'
              variant='contained'
              onClick={handleSaveEdit}
            >
              {t("update")}
            </Button>
          </Box>
        </Box>
      </Fade>
    </Modal>
  );
}
