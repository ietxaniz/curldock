import VerticalSplitPane from "./components/VerticalSplitPane";
import Designer from "./components/Designer";
import FileExplorer from "./components/FileExplorer";

const App = () => {
  return (
    <div className="h-screen w-screen">
      <VerticalSplitPane minLeftWidth={2} maxLeftWidth={90} initial={15} barWidth={5} left={<FileExplorer />} right={<Designer />} />
    </div>
  );
};

export default App;
