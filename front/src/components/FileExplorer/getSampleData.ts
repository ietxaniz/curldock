import { TreeItem } from "react-complex-tree";
import { ItemData } from "../../store/slices/fileexplorerSlice";

export const getSampleData = (): TreeItem<ItemData>[] => {
  return [
    {
      index: "root",
      isFolder: true,
      children: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
      data: { name: "Root", editing: false, idx: 0 },
    },
    {
      index: 1,
      isFolder: true,
      children: [11, 12, 13, 14, 15],
      data: { name: "Folder 1", editing: false, idx: 1 },
    },
    {
      index: 2,
      isFolder: true,
      children: [16, 17, 18, 19, 20],
      data: { name: "Folder 2", editing: false, idx: 2 },
    },
    {
      index: 3,
      isFolder: true,
      children: [21, 22],
      data: { name: "Folder 3", editing: false, idx: 3 },
    },
    {
      index: 4,
      isFolder: true,
      children: [23, 24, 25],
      data: { name: "Folder 4", editing: false, idx: 4 },
    },
    {
      index: 5,
      isFolder: true,
      children: [26, 27, 28],
      data: { name: "Folder 5", editing: false, idx: 5 },
    },
    {
      index: 6,
      isFolder: true,
      children: [29, 30, 31],
      data: { name: "Folder 6", editing: false, idx: 6 },
    },
    {
      index: 7,
      isFolder: true,
      children: [32, 33],
      data: { name: "Folder 7", editing: false, idx: 7 },
    },
    {
      index: 8,
      isFolder: true,
      children: [34, 35],
      data: { name: "Folder 8", editing: false, idx: 8 },
    },
    {
      index: 9,
      isFolder: true,
      children: [36, 37],
      data: { name: "Folder 9", editing: false, idx: 9 },
    },
    {
      index: 10,
      isFolder: true,
      children: [38, 39],
      data: { name: "Folder 10", editing: false, idx: 10 },
    },
    {
      index: 11,
      isFolder: false,
      data: { name: "File 1", editing: false, idx: 11 },
    },
    {
      index: 12,
      isFolder: false,
      data: { name: "File 2", editing: false, idx: 12 },
    },
    {
      index: 13,
      isFolder: false,
      data: { name: "File 3", editing: false, idx: 13 },
    },
    {
      index: 14,
      isFolder: false,
      data: { name: "File 4", editing: false, idx: 14 },
    },
    {
      index: 15,
      isFolder: false,
      data: { name: "File 5", editing: false, idx: 15 },
    },
    {
      index: 16,
      isFolder: false,
      data: { name: "File 6", editing: false, idx: 16 },
    },
    {
      index: 17,
      isFolder: false,
      data: { name: "File 7", editing: false, idx: 17 },
    },
    {
      index: 18,
      isFolder: false,
      data: { name: "File 8", editing: false, idx: 18 },
    },
    {
      index: 19,
      isFolder: false,
      data: { name: "File 9", editing: false, idx: 19 },
    },
    {
      index: 20,
      isFolder: false,
      data: { name: "File 10", editing: false, idx: 20 },
    },
    {
      index: 21,
      isFolder: false,
      data: { name: "File 11", editing: false, idx: 21 },
    },
    {
      index: 22,
      isFolder: false,
      data: { name: "File 12", editing: false, idx: 22 },
    },
    {
      index: 23,
      isFolder: false,
      data: { name: "File 13", editing: false, idx: 23 },
    },
    {
      index: 24,
      isFolder: false,
      data: { name: "File 14", editing: false, idx: 24 },
    },
    {
      index: 25,
      isFolder: false,
      data: { name: "File 15", editing: false, idx: 25 },
    },
    {
      index: 26,
      isFolder: false,
      data: { name: "File 16", editing: false, idx: 26 },
    },
    {
      index: 27,
      isFolder: false,
      data: { name: "File 17", editing: false, idx: 27 },
    },
    {
      index: 28,
      isFolder: false,
      data: { name: "File 18", editing: false, idx: 28 },
    },
    {
      index: 29,
      isFolder: false,
      data: { name: "File 19", editing: false, idx: 29 },
    },
    {
      index: 30,
      isFolder: false,
      data: { name: "File 20", editing: false, idx: 30 },
    },
    {
      index: 31,
      isFolder: false,
      data: { name: "File 21", editing: false, idx: 31 },
    },
    {
      index: 32,
      isFolder: false,
      data: { name: "File 22", editing: false, idx: 32 },
    },
    {
      index: 33,
      isFolder: false,
      data: { name: "File 23", editing: false, idx: 33 },
    },
    {
      index: 34,
      isFolder: false,
      data: { name: "File 24", editing: false, idx: 34 },
    },
    {
      index: 35,
      isFolder: false,
      data: { name: "File 25", editing: false, idx: 35 },
    },
    {
      index: 36,
      isFolder: false,
      data: { name: "File 26", editing: false, idx: 36 },
    },
    {
      index: 37,
      isFolder: false,
      data: { name: "File 27", editing: false, idx: 37 },
    },
    {
      index: 38,
      isFolder: false,
      data: { name: "File 28", editing: false, idx: 38 },
    },
    {
      index: 39,
      isFolder: false,
      data: { name: "File 29", editing: false, idx: 39 },
    }
  ];
}
