/****************************************************************************
** Meta object code from reading C++ file 'gamesettingswidget.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/gamesettingswidget.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'gamesettingswidget.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_GameSettingsWidget_t {
    QByteArrayData data[12];
    char stringdata0[143];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_GameSettingsWidget_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_GameSettingsWidget_t qt_meta_stringdata_GameSettingsWidget = {
    {
QT_MOC_LITERAL(0, 0, 18), // "GameSettingsWidget"
QT_MOC_LITERAL(1, 19, 14), // "variantChanged"
QT_MOC_LITERAL(2, 34, 0), // ""
QT_MOC_LITERAL(3, 35, 7), // "variant"
QT_MOC_LITERAL(4, 43, 18), // "timeControlChanged"
QT_MOC_LITERAL(5, 62, 13), // "statusChanged"
QT_MOC_LITERAL(6, 76, 2), // "ok"
QT_MOC_LITERAL(7, 79, 19), // "onHumanCountChanged"
QT_MOC_LITERAL(8, 99, 5), // "count"
QT_MOC_LITERAL(9, 105, 11), // "validateFen"
QT_MOC_LITERAL(10, 117, 3), // "fen"
QT_MOC_LITERAL(11, 121, 21) // "showTimeControlDialog"

    },
    "GameSettingsWidget\0variantChanged\0\0"
    "variant\0timeControlChanged\0statusChanged\0"
    "ok\0onHumanCountChanged\0count\0validateFen\0"
    "fen\0showTimeControlDialog"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_GameSettingsWidget[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       6,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       3,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    1,   44,    2, 0x06 /* Public */,
       4,    0,   47,    2, 0x06 /* Public */,
       5,    1,   48,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
       7,    1,   51,    2, 0x0a /* Public */,
       9,    1,   54,    2, 0x08 /* Private */,
      11,    0,   57,    2, 0x08 /* Private */,

 // signals: parameters
    QMetaType::Void, QMetaType::QString,    3,
    QMetaType::Void,
    QMetaType::Void, QMetaType::Bool,    6,

 // slots: parameters
    QMetaType::Void, QMetaType::Int,    8,
    QMetaType::Void, QMetaType::QString,   10,
    QMetaType::Void,

       0        // eod
};

void GameSettingsWidget::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<GameSettingsWidget *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->variantChanged((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 1: _t->timeControlChanged(); break;
        case 2: _t->statusChanged((*reinterpret_cast< bool(*)>(_a[1]))); break;
        case 3: _t->onHumanCountChanged((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 4: _t->validateFen((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 5: _t->showTimeControlDialog(); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (GameSettingsWidget::*)(const QString & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&GameSettingsWidget::variantChanged)) {
                *result = 0;
                return;
            }
        }
        {
            using _t = void (GameSettingsWidget::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&GameSettingsWidget::timeControlChanged)) {
                *result = 1;
                return;
            }
        }
        {
            using _t = void (GameSettingsWidget::*)(bool );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&GameSettingsWidget::statusChanged)) {
                *result = 2;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject GameSettingsWidget::staticMetaObject = { {
    QMetaObject::SuperData::link<QWidget::staticMetaObject>(),
    qt_meta_stringdata_GameSettingsWidget.data,
    qt_meta_data_GameSettingsWidget,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *GameSettingsWidget::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *GameSettingsWidget::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_GameSettingsWidget.stringdata0))
        return static_cast<void*>(this);
    return QWidget::qt_metacast(_clname);
}

int GameSettingsWidget::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QWidget::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 6)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 6;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 6)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 6;
    }
    return _id;
}

// SIGNAL 0
void GameSettingsWidget::variantChanged(const QString & _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}

// SIGNAL 1
void GameSettingsWidget::timeControlChanged()
{
    QMetaObject::activate(this, &staticMetaObject, 1, nullptr);
}

// SIGNAL 2
void GameSettingsWidget::statusChanged(bool _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 2, _a);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
