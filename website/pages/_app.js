import "../styles/globals.css";
import Head from "next/head";

function MyApp({ Component, pageProps }) {
  return (
    <>
      <Head>
        <title>CodeOpen</title>
        <meta charSet="UTF-8" />
        <meta property="og:image" content="/mango.png" />
        <meta
          name="description"
          content="A productivity tool that helps you to open your programming projects faster and simpler, directly from the command line!"
        />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <link rel="shortcut icon" href="/favicon.ico" />
      </Head>
      <Component {...pageProps} />
    </>
  );
}

export default MyApp;
