import { MUSL, familySync } from "detect-libc";
type NativeBinding = typeof import("./binding");

function requireNative(): NativeBinding {
  let parts: string[] = [process.platform, process.arch];
  if (process.platform === "linux") {
    if (familySync() === MUSL) {
      parts.push("musl");
    } else if (process.arch === "arm") {
      parts.push("gnueabihf");
    } else {
      parts.push("gnu");
    }
  } else if (process.platform === "win32") {
    parts.push("msvc");
  }

  try {
    return require(`@rswind/binding-${parts.join("-")}`);
  } catch (err) {
    const binding =`./rswind.${parts.join("-")}.node`;
    // explicitly extract `binding` to avoid glob import
    return require(binding);
  }
}

const binding = requireNative();

export default binding;