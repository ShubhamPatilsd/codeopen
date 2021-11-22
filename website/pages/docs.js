import ReactMarkdown from "react-markdown";

export default function Docs(props) {
  return (
    <div className="p-8 md:p-16">
      <ReactMarkdown>{props.markdown}</ReactMarkdown>
    </div>
  );
}

export async function getStaticProps() {
  const data = await fetch(
    "https://raw.githubusercontent.com/ShubhamPatilsd/codeopen/main/docs/guide.md"
  );

  const parsed_data = await data.text();

  return { props: { markdown: parsed_data } };
}
