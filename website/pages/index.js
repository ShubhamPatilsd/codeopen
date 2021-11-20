export default function Home() {
  return (
    <div>
      <div className="space-y-4">
        <h1 className="text-center font-black">CodeOpen</h1>
        <p className="text-center">No more of this:</p>
        <div class="flex justify-center">
          <div className="bg-gray-200 rounded-md p-2 font-mono w-max">
            > cd ~/really/long/path/to/project
            <br /> > code .
          </div>{" "}
        </div>
      </div>
    </div>
  );
}
