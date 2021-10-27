import * as React from "react";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import ProjectRow from "./ProjectRow";
import { Box } from "@mui/system";

function createData(name: string, created: string) {
  return {
    name,
    created,
    renders: [
      {
        renderName: name + "-project-render-1",
        status: 100, //percentage
        renderTime: "1 h 40 min 23 sec",
      },
      {
        renderName: name + "-project-render-2",
        status: 45, //percentage
        renderTime: "35 min 21 sec",
      },
    ],
  };
}

const rows = [
  createData("Nice house", "2021.10.25"),
  createData("Another nice house", "2021.10.26"),
  createData("An unfinished project", "2020.10.30"),
  createData("An unfinished project", "2020.10.30"),
  createData("An unfinished project", "2020.10.30"),
  createData("An unfinished project", "2020.10.30"),
  createData("An unfinished project", "2020.10.30"),
  createData("An unfinished project", "2020.10.30"),
  createData("An unfinished project", "2020.10.30"),
  createData("An unfinished project", "2020.10.30"),
  createData("An unfinished project", "2020.10.30"),
];

export default function CollapsibleTable() {
  const [expanded, setExpanded] = React.useState<number | false>(false);

  const handleChange =
    (row: number) => (event: React.SyntheticEvent, isExpanded: boolean) => {
      setExpanded(isExpanded ? row : false);
    };

  return (
    //TODO: make the container the same height as its parent component
    <TableContainer sx={{ maxHeight: "600px" }}>
      <Table stickyHeader>
        <TableHead>
          <TableRow>
            <TableCell width='10%'>
              <Box height='24px' width='24px' padding='10px' />
            </TableCell>
            <TableCell>NAME</TableCell>
            <TableCell align='right'>CREATED</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          <TableRow>
            <TableCell padding='none' colSpan={3}>
              {rows.map((row, index) => (
                <ProjectRow
                  key={index}
                  id={index}
                  row={row}
                  expanded={expanded}
                  handleChange={handleChange}
                />
              ))}
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </TableContainer>
  );
}
