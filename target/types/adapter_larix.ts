export type AdapterLarix = {
  "version": "0.1.0",
  "name": "adapter_larix",
  "instructions": [
    {
      "name": "supply",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "unsupply",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "borrow",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "repay",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "stake",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "unstake",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "collateralize",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "uncollateralize",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "harvest",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimCollateralReward",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "types": [
    {
      "name": "UnstakeResultWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "GatewayStateWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "discriminator",
            "type": "u64"
          },
          {
            "name": "userKey",
            "type": "publicKey"
          },
          {
            "name": "randomSeed",
            "type": "u64"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "currentIndex",
            "type": "u8"
          },
          {
            "name": "queueSize",
            "type": "u8"
          },
          {
            "name": "protocolQueue",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "actionQueue",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "versionQueue",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "payloadQueue",
            "type": {
              "array": [
                "u64",
                8
              ]
            }
          },
          {
            "name": "swapMinOutAmount",
            "type": "u64"
          },
          {
            "name": "poolDirection",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnsupportedAction",
      "msg": "Unsupported Action"
    }
  ]
};

export const IDL: AdapterLarix = {
  "version": "0.1.0",
  "name": "adapter_larix",
  "instructions": [
    {
      "name": "supply",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "unsupply",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "borrow",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "repay",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "stake",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "unstake",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "collateralize",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "uncollateralize",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "harvest",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimCollateralReward",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "types": [
    {
      "name": "UnstakeResultWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "GatewayStateWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "discriminator",
            "type": "u64"
          },
          {
            "name": "userKey",
            "type": "publicKey"
          },
          {
            "name": "randomSeed",
            "type": "u64"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "currentIndex",
            "type": "u8"
          },
          {
            "name": "queueSize",
            "type": "u8"
          },
          {
            "name": "protocolQueue",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "actionQueue",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "versionQueue",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "payloadQueue",
            "type": {
              "array": [
                "u64",
                8
              ]
            }
          },
          {
            "name": "swapMinOutAmount",
            "type": "u64"
          },
          {
            "name": "poolDirection",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnsupportedAction",
      "msg": "Unsupported Action"
    }
  ]
};
