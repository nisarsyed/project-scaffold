export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <h1 className="text-4xl font-bold mb-4">{{project_name}}</h1>
      <p className="text-gray-600 dark:text-gray-400">{{description}}</p>
      <div className="mt-8 flex gap-4">
        <a
          href="https://nextjs.org/docs"
          className="rounded-lg bg-black px-4 py-2 text-white hover:bg-gray-800 dark:bg-white dark:text-black dark:hover:bg-gray-200"
        >
          Next.js Docs
        </a>
        <a
          href="https://tailwindcss.com/docs"
          className="rounded-lg border border-gray-300 px-4 py-2 hover:bg-gray-100 dark:border-gray-700 dark:hover:bg-gray-800"
        >
          Tailwind Docs
        </a>
      </div>
    </main>
  );
}
