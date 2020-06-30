#!/bin/bash
_home="/"
if [[ ${_home} == "/" ]]; then
    _home=""
fi

echo "${_home}"/.cargo/bin
