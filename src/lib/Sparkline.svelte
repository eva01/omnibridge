<script lang="ts">
  let {
    values,
    color = '#4f6ef7',
    height = 24,
  }: {
    values: number[];
    color?: string;
    height?: number;
  } = $props();

  const hasData = $derived(values.length >= 2);
  const min = $derived(hasData ? Math.min(...values) : 0);
  const max = $derived(hasData ? Math.max(...values) : 1);
  const range = $derived(max - min || 1);

  const polyline = $derived(
    hasData
      ? values
          .map((v, i) => {
            const x = (i / (values.length - 1)) * 100;
            const y = 100 - ((v - min) / range) * 80 - 10;
            return `${x.toFixed(2)},${y.toFixed(2)}`;
          })
          .join(' ')
      : ''
  );

  const areaPath = $derived(
    hasData ? `M 0,100 L ${polyline.replaceAll(' ', ' L ')} L 100,100 Z` : ''
  );
</script>

<svg
  viewBox="0 0 100 100"
  preserveAspectRatio="none"
  class="spark"
  style="height: {height}px"
>
  {#if hasData}
    <defs>
      <linearGradient id="spark-grad-{color.replace('#', '')}" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color={color} stop-opacity="0.3" />
        <stop offset="100%" stop-color={color} stop-opacity="0" />
      </linearGradient>
    </defs>
    <path d={areaPath} fill="url(#spark-grad-{color.replace('#', '')})" />
    <polyline
      points={polyline}
      fill="none"
      stroke={color}
      stroke-width="1.5"
      stroke-linejoin="round"
      vector-effect="non-scaling-stroke"
    />
  {:else}
    <line x1="0" y1="50" x2="100" y2="50" stroke="#2a2a42" stroke-width="0.5" stroke-dasharray="2,2" />
  {/if}
</svg>

<style>
  .spark {
    display: block;
    width: 100%;
  }
</style>
