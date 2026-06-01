import * as Tooltip from "@radix-ui/react-tooltip";

const InfoTip = ({ children }) => {
  return (
    <Tooltip.Root delayDuration={150}>
      <Tooltip.Trigger asChild>
        <span className="info-tip" aria-label="More info">
          ⓘ
        </span>
      </Tooltip.Trigger>
      <Tooltip.Portal>
        <Tooltip.Content className="info-tip-content" side="top" sideOffset={6}>
          {children}
          <Tooltip.Arrow className="info-tip-arrow" />
        </Tooltip.Content>
      </Tooltip.Portal>
    </Tooltip.Root>
  );
};

export default InfoTip;
