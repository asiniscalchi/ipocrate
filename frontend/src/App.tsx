import { Activity } from "lucide-react"

import { Button } from "@/components/ui/button"

function App() {
  return (
    <main className="min-h-svh bg-background text-foreground">
      <header className="border-b border-border">
        <div className="mx-auto flex h-14 w-full max-w-5xl items-center justify-between px-6">
          <div className="text-sm font-medium">Ipocrate</div>
          <Button asChild variant="outline" size="sm">
            <a href="/health">
              <Activity aria-hidden="true" />
              Health
            </a>
          </Button>
        </div>
      </header>

      <section className="mx-auto flex min-h-[calc(100svh-3.5rem)] w-full max-w-5xl flex-col justify-center px-6 py-16">
        <div className="max-w-2xl space-y-6">
          <div className="inline-flex items-center rounded-lg border border-border bg-muted px-3 py-1 text-sm text-muted-foreground">
            Workspace ready
          </div>
          <div className="space-y-3">
            <h1 className="text-4xl font-semibold tracking-normal sm:text-5xl">
              Ipocrate
            </h1>
            <p className="max-w-xl text-base leading-7 text-muted-foreground">
              The first application shell is available from the backend.
            </p>
          </div>
        </div>
      </section>
    </main>
  )
}

export default App
