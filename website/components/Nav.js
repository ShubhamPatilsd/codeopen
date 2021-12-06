import { FaGithub } from "react-icons/fa";
export const Navbar = () => {
  return (
    <div className="flex justify-between py-4 px-8">
      <div className="space-x-4 flex">
        <h4 className="transition duration-150 border-b-2 border-transparent hover:border-white w-max cursor-pointer">
          <a href="/">Home</a>
        </h4>
        <h4 className="transition duration-150 border-b-2 border-transparent hover:border-white w-max cursor-pointer">
          <a href="/docs">Docs</a>
        </h4>
      </div>
      <div className="flex">
        <a
          href="https://github.com/ShubhamPatilsd/codeopen/"
          className="transform hover:-translate-y-1 transition ease-in-out duration"
        >
          <FaGithub size={35} />
        </a>
      </div>
    </div>
  );
};
