import { IconEdit } from "@tabler/icons-react";
import { useState } from "react";
import EditRemoteModal from "./edit-remote-modal";

export default function EditRemoteIcon({ value }: { value: string }) {
  const [showModal, setShowModal] = useState(false);
  return (
    <>
      <EditRemoteModal opened={showModal} close={() => setShowModal(false)} value={value} />
      <IconEdit size={18} style={{ cursor: "pointer" }} onClick={() => setShowModal(true)} />
    </>
  );
}
