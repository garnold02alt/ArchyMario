import React from "react";
import {
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  SwipeableDrawer,
  Typography,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import { Home, Login, People } from "@mui/icons-material";
import ArchytexIcon from "../ArchytexIcon";

const DrawerHeader = styled("div")(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "flex-end",
  padding: theme.spacing(0, 1),
  // necessary for content to be below app bar
  ...theme.mixins.toolbar,
}));

interface SwipeableDrawerProps {
  open: boolean;
  handleOpenChange: (value: boolean) => void;
}
const buttonList = [
  {
    text: "Home",
    icon: <Home />,
  },
  {
    text: "Community",
    icon: <People />,
  },
  {
    text: "Login",
    icon: <Login />,
  },
];

export default function DashboardSwipeableDrawer({
  open,
  handleOpenChange,
}: SwipeableDrawerProps) {
  return (
    <SwipeableDrawer
      sx={{ display: { xs: "flex", md: "none" } }}
      anchor='left'
      open={open}
      elevation={0}
      onClose={() => handleOpenChange(false)}
      onOpen={() => handleOpenChange(true)}
    >
      <DrawerHeader sx={{ width: 300 }} />
      <DrawerHeader
        sx={{
          width: 300,
          height: 100,
          display: "flex",
          justifyContent: "center",
          backgroundSize: "10px 10px",
          backgroundImage: "radial-gradient(#1c517a .75px, #0c0c0c .75px)",
        }}
      >
        <ArchytexIcon />
        <Typography variant='h6'>Archytex</Typography>
      </DrawerHeader>
      <List>
        {buttonList.map((props, index) => (
          <ListItem
            sx={{
              borderRadius: "2px",
            }}
            button
            key={index}
          >
            <ListItemIcon>{props.icon}</ListItemIcon>
            <ListItemText primary={props.text} />
          </ListItem>
        ))}
      </List>
    </SwipeableDrawer>
  );
}
