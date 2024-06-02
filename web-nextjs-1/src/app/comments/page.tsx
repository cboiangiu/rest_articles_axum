import "./styles.css";

export default function Comments() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center overflow-hidden">
      <div className="z-10 w-full max-w-5xl items-center justify-center font-mono text-sm lg:flex">
        <p className="flex items-center w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30">
          This should be a comments section.&nbsp;
          <code className="font-mono font-bold">Hello from Next.js!</code>
        </p>
      </div>
    </main>
  );
}
