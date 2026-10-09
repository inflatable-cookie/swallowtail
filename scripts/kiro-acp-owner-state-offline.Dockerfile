FROM ubuntu@sha256:534baea6a22c03a63003dbc8dbe78fe34bc0d7e595d9a9dc9834884ff530eb55

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates strace python3 iproute2 procps util-linux xz-utils \
 && rm -rf /var/lib/apt/lists/* \
 && useradd --uid 21001 --no-create-home --home-dir /scratch/home/owner --shell /bin/sh proof-owner \
 && useradd --uid 21002 --no-create-home --home-dir /scratch/home/other --shell /bin/sh proof-other \
 && mkdir -p /opt/proof \
 && printf 'swallowtail kiro owner proof image sentinel\n' > /opt/proof/image-sentinel \
 && chmod 0444 /opt/proof/image-sentinel

CMD ["tail", "-f", "/dev/null"]
