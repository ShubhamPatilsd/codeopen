import ReactTypingEffect from "react-typing-effect";
import Button from "../components/Button";
import { Navbar } from "../components/Nav";

const projectNames = [
  "project_name",
  "tic-tac-toe",
  "qtile",
  "hackclub",
  "supabase",
  "hacktoberfest2021_repo",
  "egg_kernel",
];

export default function Home() {
  return (
    <>
      <Navbar />
      <div className="p-8 md:p-16">
        <div className="space-y-16">
          <div className="w-full space-y-4">
            <h1 className="text-center font-black text-5xl md:text-7xl">
              CodeOpen
            </h1>
            <h3 className="text-center font-normal text-lg md:text-2xl">
              A faster and easier way to open your programming projects from the
              command line.
            </h3>
            <div className="flex justify-center">
              <div>
                <div className="bg-gray-700 rounded-md p-2 font-mono w-max">
                  {">"} codeopen{" "}
                  <ReactTypingEffect
                    text={projectNames}
                    eraseSpeed={100}
                    eraseDelay={600}
                    speed={100}
                  />
                </div>
              </div>
            </div>
            <div className="flex justify-center">
              <Button additionalClass="bg-gray-100 hover:bg-gray-300 text-black">
                Let's get started!
              </Button>
            </div>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-3">
            <div className="text-center space-y-2">
              <p className="text-3xl md:text-5xl">💻</p>
              <h4>Efficient</h4>
              <p>
                Designed so that you can get to work without the mundane work
                in-between
              </p>
            </div>

            <div className="text-center space-y-2">
              <p className="text-3xl md:text-5xl">⚡️</p>
              <h4>Speedy</h4>
              <p>
                Written with the speed and excellence of the Rust programming
                language
              </p>
            </div>

            <div className="text-center space-y-2">
              <p className="text-3xl md:text-5xl">🌭</p>
              <h4>Simple</h4>
              <p>
                With a simple interface and minimal configuration, even a hotdog
                could do it!
              </p>
            </div>
          </div>
          <div>
            <h3>Overview</h3>
            <p>
              I've always found that it takes a bunch of effort and a long time
              typing file paths. To open up a file in VSCode, I would do
              something like{" "}
              <code className="bg-gray-600 rounded-md p-1">
                code ~/Documents/programming/really/long/file/path
              </code>
              . This became just a tiny tiny bit irritating and took me a bit to
              open up my projects. And since I'm just that bit lazy, I made
              CodeOpen. Now, you no longer have to type that really long
              command; instead you can just do{" "}
              <code className="bg-gray-600 rounded-md p-1">
                codeopen project
              </code>
              The project argument is something that you specify in the config
              file. To read more on the config file,{" "}
              <a href="/docs" className="text-blue-400 hover:underline">
                read the docs!
              </a>
            </p>
          </div>
        </div>
      </div>
    </>
  );
}
