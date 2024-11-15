// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { PropsWithChildren, useState } from 'react';
import {
    AccordionHeaderProps,
    Accordion,
    AccordionHeader,
    AccordionContent,
    TitleSize,
    Title,
} from '@iota/apps-ui-kit';

type CollapsibleProps = {
    title: string;
    hideBorder?: boolean;
    defaultOpen?: boolean;
    headerProps?: AccordionHeaderProps;
    titleSize?: TitleSize;
};

export function Collapsible({
    title,
    children,
    defaultOpen,
    hideBorder,
    titleSize = TitleSize.Small,
}: PropsWithChildren<CollapsibleProps>) {
    const [open, setOpen] = useState(defaultOpen ?? false);
    return (
        <Accordion hideBorder={hideBorder}>
            <AccordionHeader
                hideBorder={hideBorder}
                isExpanded={open}
                onToggle={() => setOpen(!open)}
            >
                <Title size={titleSize} title={title} />
            </AccordionHeader>
            <AccordionContent isExpanded={open}>{children}</AccordionContent>
        </Accordion>
    );
}
