/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './index.html',
    './src/**/*.vue',
  ],
  theme: {
    extend: {
      colors: {
        'smoky': {
          '50': '#f9f8fb',
          '100': '#f4f1f6',
          '200': '#e9e3eb',
          '300': '#d6ccdb',
          '400': '#beadc5',
          '500': '#a08baa',
          '600': '#836d8c',
          '700': '#68556f',
          '800': '#59495f',
          '900': '#4c3f50',
          '950': '#2c2230',
        },
      },
    },
  },
  plugins: [],
}

