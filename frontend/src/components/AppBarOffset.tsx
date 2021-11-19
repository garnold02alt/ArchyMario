import React from "react";
import { styled } from "@mui/material/styles";

const Offset = styled("div")(({ theme }) => ({
  height: `56px`,
  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    height: `48px`,
  },
  [theme.breakpoints.up("sm")]: {
    height: `64px`,
  },
}));

export default function AppBarOffset() {
  return <Offset />;
}
