"use client";

import * as React from "react";
import { Check, Moon, Sun } from "lucide-react";
import { useTheme } from "next-themes";

import { Button } from "@workspace/ui/components/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@workspace/ui/components/dropdown-menu";

export function ModeToggle() {
  const { setTheme } = useTheme();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" size="icon">
          <Sun className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
          <Moon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
          <span className="sr-only">Toggle theme</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <MenuThemeItem theme="system" setThemeAction={setTheme} />
        <MenuThemeItem theme="light" setThemeAction={setTheme} />
        <MenuThemeItem theme="dark" setThemeAction={setTheme} />
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

type MenuThemeItemProps = {
  theme: string;
  setThemeAction: (theme: string) => void;
};

export const MenuThemeItem = ({
  theme,
  setThemeAction,
}: MenuThemeItemProps) => {
  const { theme: currentTheme } = useTheme();
  return (
    <DropdownMenuItem
      className="capitalize"
      onClick={() => setThemeAction(theme)}
    >
      {theme}

      {currentTheme === theme && (
        <span className="absolute right-0 mr-2">
          <Check className="h-4 w-4" />
        </span>
      )}
    </DropdownMenuItem>
  );
};
