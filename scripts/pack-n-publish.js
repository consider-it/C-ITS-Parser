const { spawn } = require("child_process");
const { writeFileSync } = require("fs");

const build = spawn("wasm-pack", ["build", "--release", "--target", "web"]);

const addOutput = (cmd) => {
  cmd.stdout.on("data", (data) => {
    console.log(`${data}`);
  });

  cmd.stderr.on("data", (data) => {
    console.log(`${data}`);
  });

  cmd.on("error", (error) => {
    console.log(`${error.message}`);
  });
};

addOutput(build);

build.on("close", (code) => {
  if (code !== 0) return;
  const packageJson = require("../pkg/package.json");
  packageJson.name = "@consider-it/" + packageJson.name;
  packageJson.publishConfig = {
    registry: "https://npm.pkg.github.com",
  };
  writeFileSync(
    "../pkg/package.json",
    JSON.stringify(packageJson, null, 2),
    "utf8"
  );
  const pack = spawn("wasm-pack", ["publish", "--tag", "next"]);
  addOutput(pack);
});
