const Button = (props) => {
  return (
    <button
      className={`p-3 rounded-md cursor-pointer transition ease-in-out duration-200 ${props.additionalClass}`}
    >
      {props.children}
    </button>
  );
};

export default Button;
