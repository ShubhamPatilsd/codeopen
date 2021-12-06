import ReactMarkdown from "react-markdown";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { nord } from "react-syntax-highlighter/dist/cjs/styles/prism";

export default function Docs(props) {
  return (
    <div className="p-8 md:p-16">
      <ReactMarkdown
        components={{
          code({ node, inline, className, children, ...props }) {
            const match = /language-(\w+)/.exec(className || "");
            return !inline && match ? (
              <SyntaxHighlighter
                children={String(children).replace(/\n$/, "")}
                style={nord}
                language={match[1]}
                PreTag="div"
                {...props}
              />
            ) : (
              <code className={className} {...props}>
                {children}
              </code>
            );
          },
        }}
      >
        {props.markdown}
      </ReactMarkdown>
    </div>
  );
}

export async function getStaticProps() {
  const data = await fetch(process.env.DOC_MD_URL);

  const parsed_data = await data.text();

  return { props: { markdown: parsed_data } };
}
