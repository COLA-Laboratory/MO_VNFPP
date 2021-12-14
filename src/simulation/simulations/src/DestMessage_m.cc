//
// Generated file, do not edit! Created by nedtool 5.5 from src/DestMessage.msg.
//

// Disable warnings about unused variables, empty switch stmts, etc:
#ifdef _MSC_VER
#  pragma warning(disable:4101)
#  pragma warning(disable:4065)
#endif

#if defined(__clang__)
#  pragma clang diagnostic ignored "-Wshadow"
#  pragma clang diagnostic ignored "-Wconversion"
#  pragma clang diagnostic ignored "-Wunused-parameter"
#  pragma clang diagnostic ignored "-Wc++98-compat"
#  pragma clang diagnostic ignored "-Wunreachable-code-break"
#  pragma clang diagnostic ignored "-Wold-style-cast"
#elif defined(__GNUC__)
#  pragma GCC diagnostic ignored "-Wshadow"
#  pragma GCC diagnostic ignored "-Wconversion"
#  pragma GCC diagnostic ignored "-Wunused-parameter"
#  pragma GCC diagnostic ignored "-Wold-style-cast"
#  pragma GCC diagnostic ignored "-Wsuggest-attribute=noreturn"
#  pragma GCC diagnostic ignored "-Wfloat-conversion"
#endif

#include <iostream>
#include <sstream>
#include "DestMessage_m.h"

namespace omnetpp {

// Template pack/unpack rules. They are declared *after* a1l type-specific pack functions for multiple reasons.
// They are in the omnetpp namespace, to allow them to be found by argument-dependent lookup via the cCommBuffer argument

// Packing/unpacking an std::vector
template<typename T, typename A>
void doParsimPacking(omnetpp::cCommBuffer *buffer, const std::vector<T,A>& v)
{
    int n = v.size();
    doParsimPacking(buffer, n);
    for (int i = 0; i < n; i++)
        doParsimPacking(buffer, v[i]);
}

template<typename T, typename A>
void doParsimUnpacking(omnetpp::cCommBuffer *buffer, std::vector<T,A>& v)
{
    int n;
    doParsimUnpacking(buffer, n);
    v.resize(n);
    for (int i = 0; i < n; i++)
        doParsimUnpacking(buffer, v[i]);
}

// Packing/unpacking an std::list
template<typename T, typename A>
void doParsimPacking(omnetpp::cCommBuffer *buffer, const std::list<T,A>& l)
{
    doParsimPacking(buffer, (int)l.size());
    for (typename std::list<T,A>::const_iterator it = l.begin(); it != l.end(); ++it)
        doParsimPacking(buffer, (T&)*it);
}

template<typename T, typename A>
void doParsimUnpacking(omnetpp::cCommBuffer *buffer, std::list<T,A>& l)
{
    int n;
    doParsimUnpacking(buffer, n);
    for (int i=0; i<n; i++) {
        l.push_back(T());
        doParsimUnpacking(buffer, l.back());
    }
}

// Packing/unpacking an std::set
template<typename T, typename Tr, typename A>
void doParsimPacking(omnetpp::cCommBuffer *buffer, const std::set<T,Tr,A>& s)
{
    doParsimPacking(buffer, (int)s.size());
    for (typename std::set<T,Tr,A>::const_iterator it = s.begin(); it != s.end(); ++it)
        doParsimPacking(buffer, *it);
}

template<typename T, typename Tr, typename A>
void doParsimUnpacking(omnetpp::cCommBuffer *buffer, std::set<T,Tr,A>& s)
{
    int n;
    doParsimUnpacking(buffer, n);
    for (int i=0; i<n; i++) {
        T x;
        doParsimUnpacking(buffer, x);
        s.insert(x);
    }
}

// Packing/unpacking an std::map
template<typename K, typename V, typename Tr, typename A>
void doParsimPacking(omnetpp::cCommBuffer *buffer, const std::map<K,V,Tr,A>& m)
{
    doParsimPacking(buffer, (int)m.size());
    for (typename std::map<K,V,Tr,A>::const_iterator it = m.begin(); it != m.end(); ++it) {
        doParsimPacking(buffer, it->first);
        doParsimPacking(buffer, it->second);
    }
}

template<typename K, typename V, typename Tr, typename A>
void doParsimUnpacking(omnetpp::cCommBuffer *buffer, std::map<K,V,Tr,A>& m)
{
    int n;
    doParsimUnpacking(buffer, n);
    for (int i=0; i<n; i++) {
        K k; V v;
        doParsimUnpacking(buffer, k);
        doParsimUnpacking(buffer, v);
        m[k] = v;
    }
}

// Default pack/unpack function for arrays
template<typename T>
void doParsimArrayPacking(omnetpp::cCommBuffer *b, const T *t, int n)
{
    for (int i = 0; i < n; i++)
        doParsimPacking(b, t[i]);
}

template<typename T>
void doParsimArrayUnpacking(omnetpp::cCommBuffer *b, T *t, int n)
{
    for (int i = 0; i < n; i++)
        doParsimUnpacking(b, t[i]);
}

// Default rule to prevent compiler from choosing base class' doParsimPacking() function
template<typename T>
void doParsimPacking(omnetpp::cCommBuffer *, const T& t)
{
    throw omnetpp::cRuntimeError("Parsim error: No doParsimPacking() function for type %s", omnetpp::opp_typename(typeid(t)));
}

template<typename T>
void doParsimUnpacking(omnetpp::cCommBuffer *, T& t)
{
    throw omnetpp::cRuntimeError("Parsim error: No doParsimUnpacking() function for type %s", omnetpp::opp_typename(typeid(t)));
}

}  // namespace omnetpp

namespace simulation {

// forward
template<typename T, typename A>
std::ostream& operator<<(std::ostream& out, const std::vector<T,A>& vec);

// Template rule which fires if a struct or class doesn't have operator<<
template<typename T>
inline std::ostream& operator<<(std::ostream& out,const T&) {return out;}

// operator<< for std::vector<T>
template<typename T, typename A>
inline std::ostream& operator<<(std::ostream& out, const std::vector<T,A>& vec)
{
    out.put('{');
    for(typename std::vector<T,A>::const_iterator it = vec.begin(); it != vec.end(); ++it)
    {
        if (it != vec.begin()) {
            out.put(','); out.put(' ');
        }
        out << *it;
    }
    out.put('}');
    
    char buf[32];
    sprintf(buf, " (size=%u)", (unsigned int)vec.size());
    out.write(buf, strlen(buf));
    return out;
}

Register_Class(DestMessage)

DestMessage::DestMessage(const char *name, short kind) : ::omnetpp::cMessage(name,kind)
{
    this->srcServer = 0;
    this->isExternalMessage = false;
    this->destination = 0;
    this->position = 0;
    this->hopCount = 0;
    this->serviceId = 0;
    this->visitedSDN = false;
    this->hasFinished = false;
    this->produced = 0;
    this->queued = 0;
}

DestMessage::DestMessage(const DestMessage& other) : ::omnetpp::cMessage(other)
{
    copy(other);
}

DestMessage::~DestMessage()
{
}

DestMessage& DestMessage::operator=(const DestMessage& other)
{
    if (this==&other) return *this;
    ::omnetpp::cMessage::operator=(other);
    copy(other);
    return *this;
}

void DestMessage::copy(const DestMessage& other)
{
    this->srcServer = other.srcServer;
    this->isExternalMessage = other.isExternalMessage;
    this->destination = other.destination;
    this->position = other.position;
    this->hopCount = other.hopCount;
    this->serviceId = other.serviceId;
    this->visitedSDN = other.visitedSDN;
    this->hasFinished = other.hasFinished;
    this->produced = other.produced;
    this->queued = other.queued;
}

void DestMessage::parsimPack(omnetpp::cCommBuffer *b) const
{
    ::omnetpp::cMessage::parsimPack(b);
    doParsimPacking(b,this->srcServer);
    doParsimPacking(b,this->isExternalMessage);
    doParsimPacking(b,this->destination);
    doParsimPacking(b,this->position);
    doParsimPacking(b,this->hopCount);
    doParsimPacking(b,this->serviceId);
    doParsimPacking(b,this->visitedSDN);
    doParsimPacking(b,this->hasFinished);
    doParsimPacking(b,this->produced);
    doParsimPacking(b,this->queued);
}

void DestMessage::parsimUnpack(omnetpp::cCommBuffer *b)
{
    ::omnetpp::cMessage::parsimUnpack(b);
    doParsimUnpacking(b,this->srcServer);
    doParsimUnpacking(b,this->isExternalMessage);
    doParsimUnpacking(b,this->destination);
    doParsimUnpacking(b,this->position);
    doParsimUnpacking(b,this->hopCount);
    doParsimUnpacking(b,this->serviceId);
    doParsimUnpacking(b,this->visitedSDN);
    doParsimUnpacking(b,this->hasFinished);
    doParsimUnpacking(b,this->produced);
    doParsimUnpacking(b,this->queued);
}

int DestMessage::getSrcServer() const
{
    return this->srcServer;
}

void DestMessage::setSrcServer(int srcServer)
{
    this->srcServer = srcServer;
}

bool DestMessage::getIsExternalMessage() const
{
    return this->isExternalMessage;
}

void DestMessage::setIsExternalMessage(bool isExternalMessage)
{
    this->isExternalMessage = isExternalMessage;
}

int DestMessage::getDestination() const
{
    return this->destination;
}

void DestMessage::setDestination(int destination)
{
    this->destination = destination;
}

int DestMessage::getPosition() const
{
    return this->position;
}

void DestMessage::setPosition(int position)
{
    this->position = position;
}

int DestMessage::getHopCount() const
{
    return this->hopCount;
}

void DestMessage::setHopCount(int hopCount)
{
    this->hopCount = hopCount;
}

int DestMessage::getServiceId() const
{
    return this->serviceId;
}

void DestMessage::setServiceId(int serviceId)
{
    this->serviceId = serviceId;
}

bool DestMessage::getVisitedSDN() const
{
    return this->visitedSDN;
}

void DestMessage::setVisitedSDN(bool visitedSDN)
{
    this->visitedSDN = visitedSDN;
}

bool DestMessage::getHasFinished() const
{
    return this->hasFinished;
}

void DestMessage::setHasFinished(bool hasFinished)
{
    this->hasFinished = hasFinished;
}

::omnetpp::simtime_t DestMessage::getProduced() const
{
    return this->produced;
}

void DestMessage::setProduced(::omnetpp::simtime_t produced)
{
    this->produced = produced;
}

::omnetpp::simtime_t DestMessage::getQueued() const
{
    return this->queued;
}

void DestMessage::setQueued(::omnetpp::simtime_t queued)
{
    this->queued = queued;
}

class DestMessageDescriptor : public omnetpp::cClassDescriptor
{
  private:
    mutable const char **propertynames;
  public:
    DestMessageDescriptor();
    virtual ~DestMessageDescriptor();

    virtual bool doesSupport(omnetpp::cObject *obj) const override;
    virtual const char **getPropertyNames() const override;
    virtual const char *getProperty(const char *propertyname) const override;
    virtual int getFieldCount() const override;
    virtual const char *getFieldName(int field) const override;
    virtual int findField(const char *fieldName) const override;
    virtual unsigned int getFieldTypeFlags(int field) const override;
    virtual const char *getFieldTypeString(int field) const override;
    virtual const char **getFieldPropertyNames(int field) const override;
    virtual const char *getFieldProperty(int field, const char *propertyname) const override;
    virtual int getFieldArraySize(void *object, int field) const override;

    virtual const char *getFieldDynamicTypeString(void *object, int field, int i) const override;
    virtual std::string getFieldValueAsString(void *object, int field, int i) const override;
    virtual bool setFieldValueAsString(void *object, int field, int i, const char *value) const override;

    virtual const char *getFieldStructName(int field) const override;
    virtual void *getFieldStructValuePointer(void *object, int field, int i) const override;
};

Register_ClassDescriptor(DestMessageDescriptor)

DestMessageDescriptor::DestMessageDescriptor() : omnetpp::cClassDescriptor("simulation::DestMessage", "omnetpp::cMessage")
{
    propertynames = nullptr;
}

DestMessageDescriptor::~DestMessageDescriptor()
{
    delete[] propertynames;
}

bool DestMessageDescriptor::doesSupport(omnetpp::cObject *obj) const
{
    return dynamic_cast<DestMessage *>(obj)!=nullptr;
}

const char **DestMessageDescriptor::getPropertyNames() const
{
    if (!propertynames) {
        static const char *names[] = {  nullptr };
        omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
        const char **basenames = basedesc ? basedesc->getPropertyNames() : nullptr;
        propertynames = mergeLists(basenames, names);
    }
    return propertynames;
}

const char *DestMessageDescriptor::getProperty(const char *propertyname) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    return basedesc ? basedesc->getProperty(propertyname) : nullptr;
}

int DestMessageDescriptor::getFieldCount() const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    return basedesc ? 10+basedesc->getFieldCount() : 10;
}

unsigned int DestMessageDescriptor::getFieldTypeFlags(int field) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    if (basedesc) {
        if (field < basedesc->getFieldCount())
            return basedesc->getFieldTypeFlags(field);
        field -= basedesc->getFieldCount();
    }
    static unsigned int fieldTypeFlags[] = {
        FD_ISEDITABLE,
        FD_ISEDITABLE,
        FD_ISEDITABLE,
        FD_ISEDITABLE,
        FD_ISEDITABLE,
        FD_ISEDITABLE,
        FD_ISEDITABLE,
        FD_ISEDITABLE,
        FD_ISEDITABLE,
        FD_ISEDITABLE,
    };
    return (field>=0 && field<10) ? fieldTypeFlags[field] : 0;
}

const char *DestMessageDescriptor::getFieldName(int field) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    if (basedesc) {
        if (field < basedesc->getFieldCount())
            return basedesc->getFieldName(field);
        field -= basedesc->getFieldCount();
    }
    static const char *fieldNames[] = {
        "srcServer",
        "isExternalMessage",
        "destination",
        "position",
        "hopCount",
        "serviceId",
        "visitedSDN",
        "hasFinished",
        "produced",
        "queued",
    };
    return (field>=0 && field<10) ? fieldNames[field] : nullptr;
}

int DestMessageDescriptor::findField(const char *fieldName) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    int base = basedesc ? basedesc->getFieldCount() : 0;
    if (fieldName[0]=='s' && strcmp(fieldName, "srcServer")==0) return base+0;
    if (fieldName[0]=='i' && strcmp(fieldName, "isExternalMessage")==0) return base+1;
    if (fieldName[0]=='d' && strcmp(fieldName, "destination")==0) return base+2;
    if (fieldName[0]=='p' && strcmp(fieldName, "position")==0) return base+3;
    if (fieldName[0]=='h' && strcmp(fieldName, "hopCount")==0) return base+4;
    if (fieldName[0]=='s' && strcmp(fieldName, "serviceId")==0) return base+5;
    if (fieldName[0]=='v' && strcmp(fieldName, "visitedSDN")==0) return base+6;
    if (fieldName[0]=='h' && strcmp(fieldName, "hasFinished")==0) return base+7;
    if (fieldName[0]=='p' && strcmp(fieldName, "produced")==0) return base+8;
    if (fieldName[0]=='q' && strcmp(fieldName, "queued")==0) return base+9;
    return basedesc ? basedesc->findField(fieldName) : -1;
}

const char *DestMessageDescriptor::getFieldTypeString(int field) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    if (basedesc) {
        if (field < basedesc->getFieldCount())
            return basedesc->getFieldTypeString(field);
        field -= basedesc->getFieldCount();
    }
    static const char *fieldTypeStrings[] = {
        "int",
        "bool",
        "int",
        "int",
        "int",
        "int",
        "bool",
        "bool",
        "simtime_t",
        "simtime_t",
    };
    return (field>=0 && field<10) ? fieldTypeStrings[field] : nullptr;
}

const char **DestMessageDescriptor::getFieldPropertyNames(int field) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    if (basedesc) {
        if (field < basedesc->getFieldCount())
            return basedesc->getFieldPropertyNames(field);
        field -= basedesc->getFieldCount();
    }
    switch (field) {
        default: return nullptr;
    }
}

const char *DestMessageDescriptor::getFieldProperty(int field, const char *propertyname) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    if (basedesc) {
        if (field < basedesc->getFieldCount())
            return basedesc->getFieldProperty(field, propertyname);
        field -= basedesc->getFieldCount();
    }
    switch (field) {
        default: return nullptr;
    }
}

int DestMessageDescriptor::getFieldArraySize(void *object, int field) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    if (basedesc) {
        if (field < basedesc->getFieldCount())
            return basedesc->getFieldArraySize(object, field);
        field -= basedesc->getFieldCount();
    }
    DestMessage *pp = (DestMessage *)object; (void)pp;
    switch (field) {
        default: return 0;
    }
}

const char *DestMessageDescriptor::getFieldDynamicTypeString(void *object, int field, int i) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    if (basedesc) {
        if (field < basedesc->getFieldCount())
            return basedesc->getFieldDynamicTypeString(object,field,i);
        field -= basedesc->getFieldCount();
    }
    DestMessage *pp = (DestMessage *)object; (void)pp;
    switch (field) {
        default: return nullptr;
    }
}

std::string DestMessageDescriptor::getFieldValueAsString(void *object, int field, int i) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    if (basedesc) {
        if (field < basedesc->getFieldCount())
            return basedesc->getFieldValueAsString(object,field,i);
        field -= basedesc->getFieldCount();
    }
    DestMessage *pp = (DestMessage *)object; (void)pp;
    switch (field) {
        case 0: return long2string(pp->getSrcServer());
        case 1: return bool2string(pp->getIsExternalMessage());
        case 2: return long2string(pp->getDestination());
        case 3: return long2string(pp->getPosition());
        case 4: return long2string(pp->getHopCount());
        case 5: return long2string(pp->getServiceId());
        case 6: return bool2string(pp->getVisitedSDN());
        case 7: return bool2string(pp->getHasFinished());
        case 8: return simtime2string(pp->getProduced());
        case 9: return simtime2string(pp->getQueued());
        default: return "";
    }
}

bool DestMessageDescriptor::setFieldValueAsString(void *object, int field, int i, const char *value) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    if (basedesc) {
        if (field < basedesc->getFieldCount())
            return basedesc->setFieldValueAsString(object,field,i,value);
        field -= basedesc->getFieldCount();
    }
    DestMessage *pp = (DestMessage *)object; (void)pp;
    switch (field) {
        case 0: pp->setSrcServer(string2long(value)); return true;
        case 1: pp->setIsExternalMessage(string2bool(value)); return true;
        case 2: pp->setDestination(string2long(value)); return true;
        case 3: pp->setPosition(string2long(value)); return true;
        case 4: pp->setHopCount(string2long(value)); return true;
        case 5: pp->setServiceId(string2long(value)); return true;
        case 6: pp->setVisitedSDN(string2bool(value)); return true;
        case 7: pp->setHasFinished(string2bool(value)); return true;
        case 8: pp->setProduced(string2simtime(value)); return true;
        case 9: pp->setQueued(string2simtime(value)); return true;
        default: return false;
    }
}

const char *DestMessageDescriptor::getFieldStructName(int field) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    if (basedesc) {
        if (field < basedesc->getFieldCount())
            return basedesc->getFieldStructName(field);
        field -= basedesc->getFieldCount();
    }
    switch (field) {
        default: return nullptr;
    };
}

void *DestMessageDescriptor::getFieldStructValuePointer(void *object, int field, int i) const
{
    omnetpp::cClassDescriptor *basedesc = getBaseClassDescriptor();
    if (basedesc) {
        if (field < basedesc->getFieldCount())
            return basedesc->getFieldStructValuePointer(object, field, i);
        field -= basedesc->getFieldCount();
    }
    DestMessage *pp = (DestMessage *)object; (void)pp;
    switch (field) {
        default: return nullptr;
    }
}

} // namespace simulation

