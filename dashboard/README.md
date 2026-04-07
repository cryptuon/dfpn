# DFPN Dashboard

Vue.js web dashboard for the Deepfake Proof Network. Shows network statistics, worker/model/request data, and participant guides.

## Development

```bash
npm install
npm run dev
```

## Build

```bash
npm run build
```

## Stack

- Vue 3.5 + TypeScript + Composition API
- Tailwind CSS 4 (dark theme)
- Vue Router 4 + Pinia
- Chart.js + vue-chartjs
- Solana wallet integration (Phantom)

## Docker Deployment

The dashboard is bundled with the indexer in a single Docker image. See `Dockerfile` and `captain-definition` for CapRover deployment.
