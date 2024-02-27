<script lang="ts">
  import { createDialog } from "@melt-ui/svelte";

  let openButton;

  const {
    elements: {
      trigger: openButton,
      overlay,
      content,
      title,
      description,
      close,
      portalled,
    },
    states: { open },
  } = createDialog({ forceVisible: true });
</script>
