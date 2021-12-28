import { Category, Settings } from "@mui/icons-material";
import { Box, List, ListItemButton, Typography } from "@mui/material";
import React from "react";

export default function EditorMenu() {
  const objects = [
    {
      name: "Object",
    },
    {
      name: "Object",
    },
    {
      name: "Object",
    },
    {
      name: "Object",
    },
    {
      name: "Object",
    },
    {
      name: "Object",
    },
    {
      name: "Object",
    },
    {
      name: "Object",
    },
    {
      name: "Object",
    },
    {
      name: "Object",
    },
    {
      name: "Object",
    },
    {
      name: "Object",
    },
    {
      name: "Object",
    },
  ];

  return (
    <Box width='400px' display='flex' flexDirection='column'>
      {/* Outliner */}
      <Box borderBottom='1px solid #1F1F1F' display='flex' alignItems='center'>
        <Category
          sx={{
            marginLeft: 2,
            filter: "drop-shadow(0px 2px 4px rgba(0,0,0,0.5))  ",
          }}
        />
        <Typography marginY={1} marginLeft={1}>
          Outliner
        </Typography>
      </Box>
      <Box
        height='350px'
        borderBottom='1px solid #1F1F1F'
        sx={{ overflowY: "scroll" }}
      >
        <List>
          {objects.map((object, index) => {
            return (
              <ListItemButton key={index} sx={{ paddingLeft: 4 }}>
                {`${object.name} ${index}`}
              </ListItemButton>
            );
          })}
        </List>
      </Box>

      {/* Settings */}
      <Box borderBottom='1px solid #1F1F1F' display='flex' alignItems='center'>
        <Settings
          sx={{
            marginLeft: 2,
            filter: "drop-shadow(0px 2px 4px rgba(0,0,0,0.5))  ",
          }}
        />
        <Typography marginY={1} marginLeft={1}>
          Settings
        </Typography>
      </Box>
    </Box>
  );
}
