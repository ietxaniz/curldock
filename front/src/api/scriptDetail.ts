import { ScriptDetails } from "../store/slices/curlSlice";


export const getScriptsDetails = async (path:string) => {
  try {
    const response = await fetch("/api/v1/script-details/"+path);
    console.log(response);
    if (response.status !== 200) {
      throw "incorrect status code"
    }
    const data:ScriptDetails = await response.json();
    console.log(data);
    return data;
  } catch(error) {
    console.log(error);
  }
  return null;
}