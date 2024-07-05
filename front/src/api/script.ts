/*
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_folder: bool,
    pub path: String,
    */

export interface ScriptInfo {
  name: string;
  created_at: number;
  updated_at: number;
  is_folder: number;
  path: string;
}

interface ScriptsInfoResponse {
  path: string;
  scripts: ScriptInfo[];
}

export const getScriptsInfo = async () => {
  try {
    const response = await fetch("/api/v1/scrrecursive");
    if (response.status !== 200) {
      throw "incorrect status code"
    }
    const data:ScriptsInfoResponse = await response.json();
    return data.scripts;
  } catch(error) {
    console.log(error);
  }
  return [];
}