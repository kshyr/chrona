import {
  Home,
  Calendar,
  Search,
  Settings,
  BookMarked,
  BarChart2,
} from "lucide-react";
import { Button } from "@workspace/ui/components/button";
import { ModeToggle } from "./mode-toggle";

export function Navigation() {
  return (
    <nav className="flex h-full flex-col gap-2 p-4 [&>*]:font-semibold">
      <div className="mb-2 px-2 py-4 flex items-center justify-between">
        <h1 className="text-4xl tracking-tight font-black font-display">
          chrona.
        </h1>
        <ModeToggle />
      </div>
      <Button variant="ghost" className="w-full justify-start gap-2">
        <Home className="h-4 w-4" />
        Home
      </Button>
      <Button variant="ghost" className="w-full justify-start gap-2">
        <Calendar className="h-4 w-4" />
        Timeline
      </Button>
      <Button variant="ghost" className="w-full justify-start gap-2">
        <Search className="h-4 w-4" />
        Search
      </Button>
      <Button variant="ghost" className="w-full justify-start gap-2">
        <BookMarked className="h-4 w-4" />
        Collections
      </Button>
      <Button variant="ghost" className="w-full justify-start gap-2">
        <BarChart2 className="h-4 w-4" />
        Analytics
      </Button>
      <div className="mt-auto">
        <Button variant="ghost" className="w-full justify-start gap-2">
          <Settings className="h-4 w-4" />
          Settings
        </Button>
      </div>
    </nav>
  );
}
