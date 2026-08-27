# 240 Cline ACP Plan-Mode Evidence

Status: reserved
Owner: Tom
Created: 2026-08-27
Card: g04.085 / 241

## Question

Which exact `cline.acp` `3.0.55` value and lifecycle rows, if any, can bind
caller-selected Plan mode through the ACP path with pre-prompt application and
selected-value confirmation?

## Required Decision

Promote a closed deliver-now table or an honest empty set. Record omission and
separate requested, negotiated, configured, accepted, effective, returned, and
observed mode. Plan is behavior, not access, permission, or containment.

## Starting Evidence

Research 146 freezes the ACP `3.0.55` identity. Research 220 proves the exact
headless `--plan` path and also proves root mode argv is discarded by the ACP
early-return. ACP initialize, session options, configuration, commands, and
confirmation require their own closed route-local assessment.
