"use client";

import { useState } from "react";
import { Navigation } from "../components/navigation";
import { Editor } from "../components/editor";
import { Insights } from "../components/insights";
import { Button } from "@workspace/ui/components/button";
import { Menu } from "lucide-react";

export default function JournalPage() {
  const [showLeftSidebar, setShowLeftSidebar] = useState(true);
  const [showRightSidebar, setShowRightSidebar] = useState(true);

  return (
    <div
      className="min-h-screen bg-white dark:bg-zinc-950"
      style={{
        backgroundImage:
          "radial-gradient(circle at 1px 1px, rgb(0 0 0 / 0.05) 1px, transparent 0)",
        backgroundSize: "40px 40px",
      }}
    >
      <div className="mx-auto max-w-[1200px] px-4">
        <div className="flex min-h-screen gap-6">
          {/* Mobile Navigation Toggle */}
          <Button
            variant="ghost"
            size="icon"
            className="fixed left-4 top-4 z-50 md:hidden"
            onClick={() => setShowLeftSidebar(!showLeftSidebar)}
          >
            <Menu className="h-5 w-5" />
          </Button>

          {/* Left Sidebar */}
          <div
            className={`fixed left-0 top-0 z-40 h-full w-64 transform backdrop-blur-xl transition-transform duration-200 ease-in-out md:relative md:translate-x-0 ${
              showLeftSidebar ? "translate-x-0" : "-translate-x-full"
            }`}
          >
            <Navigation />
          </div>

          {/* Main Content */}
          <div className="flex-1 py-8 md:px-6">
            <div className="mx-auto max-w-[400px]">
              <Editor />
            </div>
          </div>

          {/* Right Sidebar */}
          <div
            className={`fixed right-0 top-0 z-40 h-full w-64 transform backdrop-blur-xl transition-transform duration-200 ease-in-out  md:relative md:translate-x-0 ${
              showRightSidebar ? "translate-x-0" : "translate-x-full"
            }`}
          >
            <Insights />
          </div>
        </div>
      </div>
    </div>
  );
}
