import React, { useRef, useState } from "react";

const VerticalSplitPane = ({
  minLeftWidth,
  maxLeftWidth,
  initial,
  left,
  right,
  barWidth = 4,
  leftOverflow,
  rightOverflow,
}: {
  minLeftWidth: number;
  maxLeftWidth: number;
  initial?: number;
  left: React.ReactNode;
  right: React.ReactNode;
  barWidth?: number;
  leftOverflow?: boolean,
  rightOverflow?:boolean,
}) => {
  const [pos, setPos] = useState(initial || 50);
  const [isResizing, setIsResizing] = useState(false);

  const divRef = useRef<HTMLDivElement>(null);

  const handleMouseDown = (e: React.MouseEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsResizing(true);
  };

  const handleMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
    e.preventDefault();
    if (!isResizing) return;
    if (!divRef.current) return;
    const divX = divRef.current.getBoundingClientRect().left;
    const divWidth = divRef.current.clientWidth;
    const newCentralPosition = e.clientX - divX - barWidth / 2;
    let newWidthPercentage = (newCentralPosition / (divWidth - barWidth)) * 100;
    if (newWidthPercentage < minLeftWidth) {
      newWidthPercentage = minLeftWidth;
    }
    if (newWidthPercentage > maxLeftWidth) {
      newWidthPercentage = maxLeftWidth;
    }
    setPos(newWidthPercentage);
  };

  const handleMouseUp = () => {
    setIsResizing(false);
  };

  const style = {
    gridTemplateColumns: `${pos}fr ${barWidth}px ${100 - pos}fr`,
  };

  return (
    <div
      ref={divRef}
      className={`h-full w-full m-0 p-0 overflow-visible grid ${isResizing ? "cursor-col-resize" : ""}`}
      style={style}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      draggable={false}
    >
      <div className={`h-full w-full ${ leftOverflow ? "overflow-auto" :""} m-0`} draggable={false}>
        {left}
      </div>
      <div
        draggable={false}
        className="bg-gray-300 border border-gray-400 border-1 w-full cursor-col-resize"
        onMouseDown={handleMouseDown}
      ></div>
      <div className={`h-full w-full ${ rightOverflow ? "overflow-auto" :""} m-0`} draggable={false}>
        {right}
      </div>
      {isResizing && <div draggable={false} className="h-full w-full absolute inset-0 bg-white opacity-0 z-50"></div>}
    </div>
  );
};

export default VerticalSplitPane;