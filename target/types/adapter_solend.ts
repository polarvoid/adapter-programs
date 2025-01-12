export type AdapterSolend = {
  "version": "0.1.0",
  "name": "adapter_solend",
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
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
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
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
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
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
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
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    }
  ],
  "types": [
    {
      "name": "SupplyInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "supplyAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnsupplyInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reservedAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "BorrowInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "borrowAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RepayInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "repayAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "SupplyOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reservedAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnsupplyOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "unsupplyAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "BorrowOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "borrowAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RepayOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "repayAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    }
  ]
};

export const IDL: AdapterSolend = {
  "version": "0.1.0",
  "name": "adapter_solend",
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
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
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
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
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
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
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
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    }
  ],
  "types": [
    {
      "name": "SupplyInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "supplyAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnsupplyInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reservedAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "BorrowInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "borrowAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RepayInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "repayAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "SupplyOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reservedAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnsupplyOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "unsupplyAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "BorrowOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "borrowAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RepayOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "repayAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
