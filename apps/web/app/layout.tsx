import { Geist, Geist_Mono, Space_Grotesk } from "next/font/google";

import "@workspace/ui/globals.css";
import { Providers } from "@/components/providers";

const fontSans = Geist({
  subsets: ["latin"],
  variable: "--font-sans",
});

const fontMono = Geist_Mono({
  subsets: ["latin"],
  variable: "--font-mono",
});

const fontDisplay = Space_Grotesk({
  subsets: ["latin"],
  variable: "--font-display",
});

export const metadata = {
  title: "Chrona",
  description: "use your journal entries to gain insights into your life",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="cdark" suppressHydrationWarning>
      <body
        className={`${fontSans.variable} ${fontMono.variable}
         ${fontDisplay.variable}
          font-sans antialiased`}
      >
        <Providers>{children}</Providers>
      </body>
    </html>
  );
}
