import { z } from "zod";

export function isValidJavaScript(jsCode: string) {
  const mySchema = z.string();
  let code = mySchema.safeParse(jsCode);
  if (!code.success) {
    throw Error("This is not a valid string");
  }

  try {
    new Function(code.data);
    return true;
  } catch (err) {
    return false;
  }
}


