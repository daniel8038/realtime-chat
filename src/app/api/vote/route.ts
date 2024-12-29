import { BN, Program } from "@coral-xyz/anchor";
import { Voting } from "../../../../anchor/target/types/voting";
import {
  ActionGetResponse,
  ActionPostRequest,
  ACTIONS_CORS_HEADERS,
  createPostResponse,
} from "@solana/actions";
import { Connection, PublicKey, Transaction } from "@solana/web3.js";
import { LinkedActionType, PostActionType } from "@solana/actions-spec";
export const OPTIONS = GET;
const IDL = require("../../../../anchor/target/idl/voting.json");
export async function GET(request: Request) {
  const actionResponse: ActionGetResponse = {
    icon: "https://zestfulkitchen.com/wp-content/uploads/2021/09/Peanut-butter_hero_for-web-2.jpg",
    title: "Vote for your favorite type of peanut butter!",
    description: "Vote between crunchy and smooth peanut butter.",
    label: "Vote",
    links: {
      actions: [
        {
          type: "transaction",
          label: "Vote for Crunchy",
          href: "/api/vote?candidate=Crunchy",
        },
        {
          type: "transaction",
          label: "Vote for Smooth",
          href: "/api/vote?candidate=Smooth",
        },
        {
          type: "message",
          label: "Vote for Smooth",
          href: "/api/vote?candidate=Smooth",
        },
      ],
    },
  };
  return Response.json(actionResponse, { headers: ACTIONS_CORS_HEADERS });
}
export async function POST(request: Request) {
  const url = new URL(request.url);
  const candidate = url.searchParams.get("candidate");
  if (candidate != "Crunchy" && candidate != "Smooth") {
    return new Response("Invalid candidate", {
      status: 400,
      headers: ACTIONS_CORS_HEADERS,
    });
  }
  const connection = new Connection("http://127.0.0.1:8899", "confirmed");
  const program: Program<Voting> = new Program(IDL, { connection });
  const body: ActionPostRequest = await request.json();
  let voter;
  let response;
  try {
    voter = new PublicKey(body.account);
  } catch (error) {
    return new Response("Invalid account", {
      status: 400,
      headers: ACTIONS_CORS_HEADERS,
    });
  }
  if (body.type === "transaction") {
    const instruction = await program.methods
      .vote(new BN(1), candidate)
      .accounts({ signer: voter })
      .instruction();
    const blockHash = await connection.getLatestBlockhash();
    const transaction = new Transaction({
      feePayer: voter,
      blockhash: blockHash.blockhash,
      lastValidBlockHeight: blockHash.lastValidBlockHeight,
    }).add(instruction);
    response = await createPostResponse({
      fields: { transaction: transaction, type: "transaction" },
    });
  }
  return Response.json(response, { headers: ACTIONS_CORS_HEADERS });
}
