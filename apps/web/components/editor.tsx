"use client";

import { useState, useRef, useEffect } from "react";
import { Button } from "@workspace/ui/components/button";
import { Card } from "@workspace/ui/components/card";
import {
  Bold,
  Italic,
  List,
  ListOrdered,
  Quote,
  Undo,
  Redo,
  ImageIcon,
  Link,
  Hash,
} from "lucide-react";
import { Separator } from "@workspace/ui/components/separator";

export function Editor() {
  const [content, setContent] = useState("");
  const [charCount, setCharCount] = useState(0);
  const [wordCount, setWordCount] = useState(0);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";
      textareaRef.current.style.height =
        textareaRef.current.scrollHeight + "px";
    }
  }, [content]);

  const handleInput = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const text = e.target.value;
    setContent(text);
    setCharCount(text.length);
    setWordCount(text.trim() === "" ? 0 : text.trim().split(/\s+/).length);
  };

  return (
    <Card className="overflow-hidden border-none bg-white/50 shadow-lg backdrop-blur-sm dark:bg-zinc-900/50">
      <div className="border-b p-2 dark:border-zinc-800">
        <div className="flex flex-wrap gap-1">
          <Button variant="ghost" size="icon" className="h-8 w-8">
            <Bold className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="icon" className="h-8 w-8">
            <Italic className="h-4 w-4" />
          </Button>
          <Separator orientation="vertical" className="mx-1 h-8" />
          <Button variant="ghost" size="icon" className="h-8 w-8">
            <List className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="icon" className="h-8 w-8">
            <ListOrdered className="h-4 w-4" />
          </Button>
          <Separator orientation="vertical" className="mx-1 h-8" />
          <Button variant="ghost" size="icon" className="h-8 w-8">
            <Quote className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="icon" className="h-8 w-8">
            <Link className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="icon" className="h-8 w-8">
            <ImageIcon className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="icon" className="h-8 w-8">
            <Hash className="h-4 w-4" />
          </Button>
          <Separator orientation="vertical" className="mx-1 h-8" />
          <Button variant="ghost" size="icon" className="h-8 w-8">
            <Undo className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="icon" className="h-8 w-8">
            <Redo className="h-4 w-4" />
          </Button>
        </div>
      </div>
      <div className="p-4">
        <div className="mb-2 font-mono text-sm text-zinc-500">
          {new Date().toLocaleString()}
        </div>
        <textarea
          ref={textareaRef}
          className="w-full resize-none bg-transparent font-sans text-lg leading-relaxed outline-none"
          placeholder="What's on your mind?"
          rows={1}
          value={content}
          onChange={handleInput}
          style={{
            fontFamily:
              '-apple-system, system-ui, BlinkMacSystemFont, "Segoe UI", Roboto',
          }}
        />
        <div className="mt-4 text-xs text-zinc-500">
          {charCount} characters · {wordCount} words
        </div>
      </div>
    </Card>
  );
}
