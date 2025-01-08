"use client";

import { useState } from "react";
import { Card } from "@workspace/ui/components/card";
import { Button } from "@workspace/ui/components/button";
import { Check, RefreshCw, X } from "lucide-react";
import { Progress } from "@workspace/ui/components/progress";

interface Insight {
  id: string;
  title: string;
  progress: number;
  total: number;
  status: "pending" | "accepted" | "rejected";
}

export function Insights() {
  const [insights, setInsights] = useState<Insight[]>([
    {
      id: "1",
      title: "Job Applications",
      progress: 2,
      total: 3,
      status: "pending",
    },
    {
      id: "2",
      title: "Writing Goals",
      progress: 500,
      total: 1000,
      status: "accepted",
    },
  ]);

  const handleAccept = (id: string) => {
    setInsights(
      insights.map((insight) =>
        insight.id === id ? { ...insight, status: "accepted" } : insight,
      ),
    );
  };

  const handleReject = (id: string) => {
    setInsights(
      insights.map((insight) =>
        insight.id === id ? { ...insight, status: "rejected" } : insight,
      ),
    );
  };

  const handleRegenerate = (id: string) => {
    // Here you would typically call an API to regenerate the insight
    console.log("Regenerating insight:", id);
  };

  return (
    <div className="h-full p-4">
      <h2 className="mb-4 text-lg font-semibold">Progress</h2>
      <div className="space-y-4">
        {insights.map((insight) => (
          <Card key={insight.id} className="p-4">
            <div className="mb-2 flex items-center justify-between">
              <h3 className="font-medium">{insight.title}</h3>
              {insight.status === "pending" && (
                <div className="flex gap-1">
                  <Button
                    variant="ghost"
                    size="icon"
                    className="h-6 w-6 text-green-600 hover:text-green-700"
                    onClick={() => handleAccept(insight.id)}
                  >
                    <Check className="h-4 w-4" />
                  </Button>
                  <Button
                    variant="ghost"
                    size="icon"
                    className="h-6 w-6 text-red-600 hover:text-red-700"
                    onClick={() => handleReject(insight.id)}
                  >
                    <X className="h-4 w-4" />
                  </Button>
                  <Button
                    variant="ghost"
                    size="icon"
                    className="h-6 w-6"
                    onClick={() => handleRegenerate(insight.id)}
                  >
                    <RefreshCw className="h-4 w-4" />
                  </Button>
                </div>
              )}
            </div>
            <Progress
              value={(insight.progress / insight.total) * 100}
              className="h-2"
            />
            <p className="mt-2 text-sm text-zinc-500">
              {insight.progress} of {insight.total} completed
            </p>
          </Card>
        ))}
      </div>
    </div>
  );
}
