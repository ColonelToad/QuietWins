import { settings } from './settings';

const themeVars: { [key: string]: { [key: string]: string } } = {
  light: {
    '--background': '#fff',
    '--text': '#222',
  },
  dark: {
    '--background': '#111',
    '--text': '#fff',
  },
  warm: {
    '--background': '#fff',
    '--text': '#CC785C',
  },
};

const fontVars: { [key: string]: string } = {
  'Garamond': "'Garamond', serif",
  'SF Pro': "'SF Pro', 'San Francisco', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Arial, sans-serif",
  'Arial': 'Arial, sans-serif',
  'Custom': 'inherit',
};

export function applyTheme(theme: string, font: string) {
  const vars = themeVars[theme] || themeVars['warm'];
  for (const k in vars) {
    document.documentElement.style.setProperty(k, vars[k]);
  }
  document.body.style.fontFamily = fontVars[font] || fontVars['Garamond'];
}

settings.subscribe(($settings) => {
  applyTheme($settings.theme, $settings.font);
});
