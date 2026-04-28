const GA_MEASUREMENT_ID = import.meta.env.VITE_GA_MEASUREMENT_ID?.trim();

declare global {
  interface Window {
    dataLayer?: unknown[];
    gtag?: (...args: unknown[]) => void;
  }
}

export type TrackedAction = {
  action: string;
  label: string;
  location: string;
  destination?: string;
};

export function initAnalytics() {
  window.dataLayer = window.dataLayer ?? [];
  if (window.gtag) return;

  window.gtag = (...args: unknown[]) => {
    window.dataLayer?.push(args);
  };

  if (!GA_MEASUREMENT_ID) return;

  const script = document.createElement("script");
  script.async = true;
  script.src = `https://www.googletagmanager.com/gtag/js?id=${encodeURIComponent(GA_MEASUREMENT_ID)}`;
  document.head.appendChild(script);

  window.gtag("js", new Date());
  window.gtag("config", GA_MEASUREMENT_ID, {
    page_title: document.title,
    page_location: window.location.href,
  });
}

export function trackAction({ action, label, location, destination }: TrackedAction) {
  const event = {
    event_category: "landing_site_action",
    event_label: label,
    link_url: destination,
    location,
  };

  window.dataLayer?.push({
    event: action,
    ...event,
  });
  window.gtag?.("event", action, event);
}
