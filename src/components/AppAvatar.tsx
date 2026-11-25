import React from "react";
import type { AppCategory } from "../types";
import { CATEGORY_MAP } from "../types";
import { CategorySvgIcon } from "./Icons";

interface AppAvatarProps {
  name: string;
  category: AppCategory;
  iconDataUrl?: string | null;
  size?: number;
  className?: string;
}

export const AppAvatar: React.FC<AppAvatarProps> = ({
  name,
  category,
  iconDataUrl,
  size = 32,
  className = "",
}) => {
  const cat = CATEGORY_MAP[category];

  if (iconDataUrl) {
    return (
      <div
        className={`shrink-0 rounded-[9px] overflow-hidden flex items-center justify-center border border-[var(--border-hairline)] bg-[var(--bg-subtle)] ${className}`}
        style={{ width: size, height: size }}
      >
        <img
          src={iconDataUrl}
          alt={name}
          className="w-[78%] h-[78%] object-contain"
          draggable={false}
        />
      </div>
    );
  }

  return (
    <div
      className={`shrink-0 rounded-[9px] flex items-center justify-center border ${className}`}
      style={{
        width: size,
        height: size,
        backgroundColor: `${cat.color}14`,
        borderColor: `${cat.color}30`,
        color: cat.color,
      }}
      title={name}
    >
      <CategorySvgIcon category={category} size={Math.round(size * 0.52)} />
    </div>
  );
};
