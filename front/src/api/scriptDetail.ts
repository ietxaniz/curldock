import { ScriptDetails } from "../store/slices/curlSlice";


interface ScriptsDetailsResponse {
  path: string;
  curl_command: ScriptDetails;
}


export const getScriptsDetails = async (path:string) => {
  try {
    const response = await fetch("/api/v1/script-details/"+path);
    console.log(response);
    if (response.status !== 200) {
      throw "incorrect status code"
    }
    const data:ScriptsDetailsResponse = await response.json();
    console.log(data);
    return data.curl_command;
  } catch(error) {
    console.log(error);
  }
  return null;
}