import React from "react";

import { useTranslation } from "react-i18next";

import { styled } from "@mui/material/styles";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import CircularProgress from "@mui/material/CircularProgress";

import { Project, useProjects } from "../../../services/projects";

import ProjectListItem from "./project-components/ProjectListItem";

const ProjectListContainer = styled(Box)(({ theme }) => ({
  height: "calc(100vh - 65px - 60px)",
  overflowY: "scroll",

  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    height: "calc(100vh - 49px - 60px - 48px)",
  },
  [theme.breakpoints.up("sm")]: {
    height: "calc(100vh - 65px - 60px - 48px)",
  },
  [theme.breakpoints.up("md")]: {
    height: "calc(100vh - 65px - 60px)",
  },
}));

interface Props {
  query: string;
}
export default function ProjectList({ query }: Props) {
  const { t } = useTranslation();

  const { projects } = useProjects();

  return (
    <ProjectListContainer>
      {projects === undefined ? (
        <Box
          height='100%'
          display='flex'
          justifyContent='center'
          alignItems='center'
          flexDirection='column'
          gap={2}
        >
          <CircularProgress />
          <Typography>{t("loading_projects")}</Typography>
        </Box>
      ) : (
        projects.map(
          (project: Project) =>
            project.title.toLowerCase().includes(query.toLowerCase()) && (
              <ProjectListItem key={project.id} project={project} />
            )
        )
      )}
    </ProjectListContainer>
  );
}
