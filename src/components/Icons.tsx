import React from "react";
import type { AppCategory, WebCategory } from "../types";

interface IconProps {
  className?: string;
  size?: number;
  strokeWidth?: number;
}

export const PrismBrandIcon: React.FC<IconProps> = ({ className = "", size = 20 }) => (
  <svg
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    className={className}
    aria-hidden="true"
  >
    <path
      d="M19.8 14.8A8.5 8.5 0 1 1 15.2 4.1"
      stroke="currentColor"
      strokeWidth="1.9"
      strokeLinecap="round"
    />
    <path
      d="M12 6.8L16.5 15.5H7.5L12 6.8Z"
      stroke="currentColor"
      strokeWidth="1.7"
      strokeLinejoin="round"
    />
    <path d="M12 6.8L13.1 15.5" stroke="#3D5AFE" strokeWidth="1.5" strokeLinecap="round" />
    <circle cx="19.2" cy="18.2" r="1.7" fill="#1A9988" />
  </svg>
);

export const OverviewNavIcon: React.FC<IconProps> = ({ className = "", size = 18, strokeWidth = 1.75 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <rect x="3.5" y="3.5" width="7" height="9" rx="2" stroke="currentColor" strokeWidth={strokeWidth} />
    <rect x="13.5" y="3.5" width="7" height="5.5" rx="2" stroke="currentColor" strokeWidth={strokeWidth} />
    <rect x="3.5" y="15" width="7" height="5.5" rx="2" stroke="currentColor" strokeWidth={strokeWidth} />
    <rect x="13.5" y="11.5" width="7" height="9" rx="2" stroke="currentColor" strokeWidth={strokeWidth} />
  </svg>
);

export const TimelineNavIcon: React.FC<IconProps> = ({ className = "", size = 18, strokeWidth = 1.75 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <path d="M3.5 6.5H20.5" stroke="currentColor" strokeWidth={strokeWidth} strokeLinecap="round" opacity="0.35" />
    <path d="M5 6.5H12.5" stroke="currentColor" strokeWidth={strokeWidth + 0.6} strokeLinecap="round" />
    <path d="M3.5 12H20.5" stroke="currentColor" strokeWidth={strokeWidth} strokeLinecap="round" opacity="0.35" />
    <path d="M9.5 12H18.5" stroke="currentColor" strokeWidth={strokeWidth + 0.6} strokeLinecap="round" />
    <path d="M3.5 17.5H20.5" stroke="currentColor" strokeWidth={strokeWidth} strokeLinecap="round" opacity="0.35" />
    <path d="M4.5 17.5H9.5M14 17.5H19.5" stroke="currentColor" strokeWidth={strokeWidth + 0.6} strokeLinecap="round" />
  </svg>
);

export const CatalogNavIcon: React.FC<IconProps> = ({ className = "", size = 18, strokeWidth = 1.75 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <path
      d="M12 3.2L19.8 7.6V16.4L12 20.8L4.2 16.4V7.6L12 3.2Z"
      stroke="currentColor"
      strokeWidth={strokeWidth}
      strokeLinejoin="round"
    />
    <path d="M4.2 7.6L12 12L19.8 7.6" stroke="currentColor" strokeWidth={strokeWidth} strokeLinejoin="round" />
    <path d="M12 12V20.8" stroke="currentColor" strokeWidth={strokeWidth} strokeLinecap="round" />
  </svg>
);

export const SettingsNavIcon: React.FC<IconProps> = ({ className = "", size = 18, strokeWidth = 1.75 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <path d="M4 7H14M18 7H20" stroke="currentColor" strokeWidth={strokeWidth} strokeLinecap="round" />
    <circle cx="16" cy="7" r="2" stroke="currentColor" strokeWidth={strokeWidth} />
    <path d="M4 12H7M11 12H20" stroke="currentColor" strokeWidth={strokeWidth} strokeLinecap="round" />
    <circle cx="9" cy="12" r="2" stroke="currentColor" strokeWidth={strokeWidth} />
    <path d="M4 17H13M17 17H20" stroke="currentColor" strokeWidth={strokeWidth} strokeLinecap="round" />
    <circle cx="15" cy="17" r="2" stroke="currentColor" strokeWidth={strokeWidth} />
  </svg>
);

export const SunIcon: React.FC<IconProps> = ({ className = "", size = 15 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <circle cx="12" cy="12" r="4.2" stroke="currentColor" strokeWidth="1.75" />
    <path
      d="M12 2.8V4.8M12 19.2V21.2M4.8 12H2.8M21.2 12H19.2M5.6 5.6L7 7M17 17L18.4 18.4M18.4 5.6L17 7M7 17L5.6 18.4"
      stroke="currentColor"
      strokeWidth="1.75"
      strokeLinecap="round"
    />
  </svg>
);

export const MoonIcon: React.FC<IconProps> = ({ className = "", size = 15 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <path
      d="M19.8 14.5A7.8 7.8 0 0 1 9.5 4.2a8.2 8.2 0 1 0 10.3 10.3Z"
      stroke="currentColor"
      strokeWidth="1.75"
      strokeLinejoin="round"
    />
  </svg>
);

export const SearchIcon: React.FC<IconProps> = ({ className = "", size = 15 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <circle cx="10.8" cy="10.8" r="6.5" stroke="currentColor" strokeWidth="1.75" />
    <path d="M15.8 15.8L20 20" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" />
  </svg>
);

export const RefreshIcon: React.FC<IconProps> = ({ className = "", size = 15 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <path
      d="M19.5 10A7.8 7.8 0 0 0 6.1 6.7L4.5 8.3M4.5 4.5V8.3H8.3"
      stroke="currentColor"
      strokeWidth="1.75"
      strokeLinecap="round"
      strokeLinejoin="round"
    />
    <path
      d="M4.5 14A7.8 7.8 0 0 0 17.9 17.3L19.5 15.7M19.5 19.5V15.7H15.7"
      stroke="currentColor"
      strokeWidth="1.75"
      strokeLinecap="round"
      strokeLinejoin="round"
    />
  </svg>
);

export const ChevronLeftIcon: React.FC<IconProps> = ({ className = "", size = 15 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <path d="M15 6L9 12L15 18" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" />
  </svg>
);

export const ChevronRightIcon: React.FC<IconProps> = ({ className = "", size = 15 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <path d="M9 6L15 12L9 18" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" />
  </svg>
);

export const ExportIcon: React.FC<IconProps> = ({ className = "", size = 15 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <path d="M12 15V4.5M12 4.5L8 8.5M12 4.5L16 8.5" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" />
    <path d="M4.5 14.5V18.5C4.5 19.6 5.4 20.5 6.5 20.5H17.5C18.6 20.5 19.5 19.6 19.5 18.5V14.5" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" />
  </svg>
);

export const EditIcon: React.FC<IconProps> = ({ className = "", size = 14 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <path
      d="M4.5 19.5H8.2L18.4 9.3a1.5 1.5 0 0 0 0-2.1l-1.6-1.6a1.5 1.5 0 0 0-2.1 0L4.5 15.8v3.7Z"
      stroke="currentColor"
      strokeWidth="1.75"
      strokeLinejoin="round"
    />
  </svg>
);

export const MinimizeWinIcon: React.FC<IconProps> = ({ className = "", size = 12 }) => (
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" className={className} aria-hidden="true">
    <path d="M3 8.5H13" stroke="currentColor" strokeWidth="1.4" strokeLinecap="round" />
  </svg>
);

export const MaximizeWinIcon: React.FC<IconProps> = ({ className = "", size = 12 }) => (
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" className={className} aria-hidden="true">
    <rect x="3.2" y="3.2" width="9.6" height="9.6" rx="1.6" stroke="currentColor" strokeWidth="1.35" />
  </svg>
);

export const CloseWinIcon: React.FC<IconProps> = ({ className = "", size = 12 }) => (
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" className={className} aria-hidden="true">
    <path d="M4 4L12 12M12 4L4 12" stroke="currentColor" strokeWidth="1.4" strokeLinecap="round" />
  </svg>
);

export const CategorySvgIcon: React.FC<{ category: AppCategory; size?: number; className?: string }> = ({
  category,
  size = 16,
  className = "",
}) => {
  switch (category) {
    case "dev":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <path d="M8.5 7.5L4 12L8.5 16.5" stroke="currentColor" strokeWidth="1.85" strokeLinecap="round" strokeLinejoin="round" />
          <path d="M15.5 7.5L20 12L15.5 16.5" stroke="currentColor" strokeWidth="1.85" strokeLinecap="round" strokeLinejoin="round" />
          <path d="M13.4 5.8L10.6 18.2" stroke="currentColor" strokeWidth="1.65" strokeLinecap="round" />
        </svg>
      );
    case "ai":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <path
            d="M12 3.5L14.3 9.7L20.5 12L14.3 14.3L12 20.5L9.7 14.3L3.5 12L9.7 9.7L12 3.5Z"
            stroke="currentColor"
            strokeWidth="1.8"
            strokeLinejoin="round"
          />
          <circle cx="12" cy="12" r="1.5" fill="currentColor" />
        </svg>
      );
    case "browser":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <circle cx="12" cy="12" r="8.2" stroke="currentColor" strokeWidth="1.75" />
          <ellipse cx="12" cy="12" rx="3.4" ry="8.2" stroke="currentColor" strokeWidth="1.5" />
          <path d="M4.2 12H19.8" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
        </svg>
      );
    case "design":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <path d="M12 4.2L19.2 12L12 19.8L4.8 12L12 4.2Z" stroke="currentColor" strokeWidth="1.75" strokeLinejoin="round" />
          <circle cx="12" cy="12" r="2.1" stroke="currentColor" strokeWidth="1.75" />
        </svg>
      );
    case "office":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <rect x="5" y="3.8" width="14" height="16.4" rx="2.4" stroke="currentColor" strokeWidth="1.75" />
          <path d="M8.8 9H15.2M8.8 12.6H15.2M8.8 16.2H12.5" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" />
        </svg>
      );
    case "social":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <path
            d="M5 6.5C5 5.4 5.9 4.5 7 4.5H17C18.1 4.5 19 5.4 19 6.5V14.2C19 15.3 18.1 16.2 17 16.2H10.2L6.2 19.5V16.2H7C5.9 16.2 5 15.3 5 14.2V6.5Z"
            stroke="currentColor"
            strokeWidth="1.75"
            strokeLinejoin="round"
          />
          <circle cx="9.2" cy="10.4" r="1" fill="currentColor" />
          <circle cx="12" cy="10.4" r="1" fill="currentColor" />
          <circle cx="14.8" cy="10.4" r="1" fill="currentColor" />
        </svg>
      );
    case "media":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <circle cx="12" cy="12" r="8.2" stroke="currentColor" strokeWidth="1.75" />
          <path d="M10.2 8.7L15.6 12L10.2 15.3V8.7Z" stroke="currentColor" strokeWidth="1.75" strokeLinejoin="round" />
        </svg>
      );
    case "game":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <rect x="3.8" y="6.8" width="16.4" height="10.4" rx="5.2" stroke="currentColor" strokeWidth="1.75" />
          <path d="M8.5 10.2V13.8M6.7 12H10.3" stroke="currentColor" strokeWidth="1.7" strokeLinecap="round" />
          <circle cx="15.3" cy="10.9" r="0.95" fill="currentColor" />
          <circle cx="17.1" cy="12.9" r="0.95" fill="currentColor" />
        </svg>
      );
    case "utility":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <rect x="4.5" y="4.5" width="6.2" height="6.2" rx="1.6" stroke="currentColor" strokeWidth="1.75" />
          <rect x="13.3" y="4.5" width="6.2" height="6.2" rx="1.6" stroke="currentColor" strokeWidth="1.75" />
          <rect x="4.5" y="13.3" width="6.2" height="6.2" rx="1.6" stroke="currentColor" strokeWidth="1.75" />
          <circle cx="16.4" cy="16.4" r="3.1" stroke="currentColor" strokeWidth="1.75" />
        </svg>
      );
  }
};

export const WebNavIcon: React.FC<IconProps> = ({ className = "", size = 18, strokeWidth = 1.75 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
    <circle cx="12" cy="12" r="8.2" stroke="currentColor" strokeWidth={strokeWidth} />
    <ellipse cx="12" cy="12" rx="3.4" ry="8.2" stroke="currentColor" strokeWidth={strokeWidth - 0.2} />
    <path d="M4.2 12H19.8" stroke="currentColor" strokeWidth={strokeWidth - 0.2} strokeLinecap="round" />
    <circle cx="17.8" cy="6.2" r="1.6" fill="currentColor" />
  </svg>
);

export const WebCategorySvgIcon: React.FC<{ category: WebCategory; size?: number; className?: string }> = ({
  category,
  size = 16,
  className = "",
}) => {
  switch (category) {
    case "dev_tech":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <path d="M8.5 7.5L4 12L8.5 16.5" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" />
          <path d="M15.5 7.5L20 12L15.5 16.5" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" />
          <path d="M13.5 5.8L10.5 18.2" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" />
        </svg>
      );
    case "ai_web":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <path
            d="M12 3.8L14.2 9.8L20.2 12L14.2 14.2L12 20.2L9.8 14.2L3.8 12L9.8 9.8L12 3.8Z"
            stroke="currentColor"
            strokeWidth="1.75"
            strokeLinejoin="round"
          />
          <circle cx="12" cy="12" r="1.4" fill="currentColor" />
        </svg>
      );
    case "video_stream":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <rect x="3.5" y="5.5" width="17" height="13" rx="3" stroke="currentColor" strokeWidth="1.75" />
          <path d="M10.2 9.2L15.2 12L10.2 14.8V9.2Z" stroke="currentColor" strokeWidth="1.65" strokeLinejoin="round" />
        </svg>
      );
    case "social_forum":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <path
            d="M4.8 6.8C4.8 5.6 5.7 4.7 6.9 4.7H17.1C18.3 4.7 19.2 5.6 19.2 6.8V14.2C19.2 15.4 18.3 16.3 17.1 16.3H10.2L6.2 19.5V16.3H6.9C5.7 16.3 4.8 15.4 4.8 14.2V6.8Z"
            stroke="currentColor"
            strokeWidth="1.75"
            strokeLinejoin="round"
          />
          <path d="M8.5 9.5H15.5M8.5 12.6H13" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" />
        </svg>
      );
    case "docs_work":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <path d="M5 4.8H10.5C11.4 4.8 12 5.4 12 6.3V19.2C12 18.4 11.4 17.8 10.5 17.8H5V4.8Z" stroke="currentColor" strokeWidth="1.7" strokeLinejoin="round" />
          <path d="M19 4.8H13.5C12.6 4.8 12 5.4 12 6.3V19.2C12 18.4 12.6 17.8 13.5 17.8H19V4.8Z" stroke="currentColor" strokeWidth="1.7" strokeLinejoin="round" />
        </svg>
      );
    case "search_portal":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <circle cx="12" cy="12" r="8" stroke="currentColor" strokeWidth="1.75" />
          <path d="M14.8 9.2L10.5 10.5L9.2 14.8L13.5 13.5L14.8 9.2Z" stroke="currentColor" strokeWidth="1.6" strokeLinejoin="round" />
        </svg>
      );
    case "shopping_life":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <path d="M5.5 8.5H18.5L17.6 19.2H6.4L5.5 8.5Z" stroke="currentColor" strokeWidth="1.75" strokeLinejoin="round" />
          <path d="M9 8.5V7C9 5.3 10.3 4 12 4C13.7 4 15 5.3 15 7V8.5" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" />
        </svg>
      );
    case "browser_internal":
      return (
        <svg width={size} height={size} viewBox="0 0 24 24" fill="none" className={className} aria-hidden="true">
          <rect x="4" y="5" width="16" height="14" rx="2.6" stroke="currentColor" strokeWidth="1.75" />
          <path d="M4 9.5H20" stroke="currentColor" strokeWidth="1.6" />
          <path d="M8.5 14H15.5" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" />
        </svg>
      );
  }
};
