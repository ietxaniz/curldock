import VerticalSplitPane from "./components/Scripts/VerticalSplitPane";
import Scripts from "./components/Scripts";
import FileExplorer from "./components/FileExplorer";

const App = () => {
  return (
    <div className="h-screen w-screen">
      <VerticalSplitPane minLeftWidth={2} maxLeftWidth={90} initial={15} barWidth={5} left={<FileExplorer />} right={<Scripts />} leftOverflow={true} />
    </div>
  );
};

export default App;
