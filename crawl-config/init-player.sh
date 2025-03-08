#!/bin/sh

RCDIR=./crawl/server/rcs-0.29/
INPROGRESSDIR=./crawl/server/rcs-0.29/running
TTYRECDIR=./crawl/server/rcs-0.29/ttyrecs/$1
DEFAULT_RC=./crawl/server/init-0.29.txt
PLAYERNAME=$1

mkdir -p $RCDIR
mkdir -p $INPROGRESSDIR
mkdir -p $TTYRECDIR

if [ ! -f ${RCDIR}/${PLAYERNAME}.rc ]; then
    cp ${DEFAULT_RC} ${RCDIR}/${PLAYERNAME}.rc
fi

RCDIR=./crawl/server/rcs-0.30/
INPROGRESSDIR=./crawl/server/rcs-0.30/running
TTYRECDIR=./crawl/server/rcs-0.30/ttyrecs/$1
DEFAULT_RC=./crawl/server/init-0.30.txt
PLAYERNAME=$1

mkdir -p $RCDIR
mkdir -p $INPROGRESSDIR
mkdir -p $TTYRECDIR

if [ ! -f ${RCDIR}/${PLAYERNAME}.rc ]; then
    cp ${DEFAULT_RC} ${RCDIR}/${PLAYERNAME}.rc
fi

RCDIR=./crawl/server/rcs-0.31/
INPROGRESSDIR=./crawl/server/rcs-0.31/running
TTYRECDIR=./crawl/server/rcs-0.31/ttyrecs/$1
DEFAULT_RC=./crawl/server/init-0.31.txt
PLAYERNAME=$1

mkdir -p $RCDIR
mkdir -p $INPROGRESSDIR
mkdir -p $TTYRECDIR

if [ ! -f ${RCDIR}/${PLAYERNAME}.rc ]; then
    cp ${DEFAULT_RC} ${RCDIR}/${PLAYERNAME}.rc
fi

RCDIR=./crawl/server/rcs-0.32/
INPROGRESSDIR=./crawl/server/rcs-0.32/running
TTYRECDIR=./crawl/server/rcs-0.32/ttyrecs/$1
DEFAULT_RC=./crawl/server/init-0.32.txt
PLAYERNAME=$1

mkdir -p $RCDIR
mkdir -p $INPROGRESSDIR
mkdir -p $TTYRECDIR

if [ ! -f ${RCDIR}/${PLAYERNAME}.rc ]; then
    cp ${DEFAULT_RC} ${RCDIR}/${PLAYERNAME}.rc
fi
