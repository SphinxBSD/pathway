import { Layout } from "@stellar/design-system";
import { Outlet } from "react-router-dom";

const App: React.FC = () => (
  <main>
    <Layout.Header
      projectId="pathway"
      projectTitle="Pathway"
    />
    <h1 className="text-3xl font-bold underline">
      Hello world!
    </h1>
    <Outlet />
    <Layout.Footer>
      <span>
        © {new Date().getFullYear()} My App. Licensed under the{" "}
        <a
          href="http://www.apache.org/licenses/LICENSE-2.0"
          target="_blank"
          rel="noopener noreferrer"
        >
          Apache License, Version 2.0
        </a>
        .
      </span>
    </Layout.Footer>
  </main>
);

export default App;
